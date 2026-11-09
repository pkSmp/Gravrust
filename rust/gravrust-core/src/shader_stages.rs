//! Port of `client/js/render/shader-stages.js`: shader stage discovery
//! and validation.
//!
//! Discovers shader stages from resource pack zips, validates
//! post-processing outputs, and checks program enable flags.

use std::collections::{HashMap, HashSet};

/// Result of shader stage discovery (port of `shaderStagePlan`).
#[derive(Debug, Clone)]
pub struct ShaderStagePlan {
    /// Post-processing stages in render order.
    pub post: Vec<String>,
    /// G-buffer geometry stages.
    pub geometry: Vec<String>,
    /// Shadow stages.
    pub shadow: Vec<String>,
    /// All discovered stage files (key -> relative path).
    pub files: HashMap<String, String>,
}

impl ShaderStagePlan {
    /// Port of `shaderStagePlan(zip, root, dimension = 0)`.
    ///
    /// Discovers shader stages from a resource pack zip.
    pub fn new(zip: &crate::zip::ZipArchive, root: &str, dimension: i32) -> Self {
        let world = match dimension {
            -1 => "world-1",
            1 => "world1",
            _ => "world0",
        };

        let mut stages = HashMap::new();

        for name in zip.list(root) {
            let rel = name.strip_prefix(root).unwrap_or(&name);
            let m = regex::Regex::new(r"^(?:(world-?\d+)/)?([^/]+)\.fsh$")
                .unwrap()
                .captures(rel);
            if let Some(caps) = m {
                let world_match = caps.get(1).map(|m| m.as_str());
                let name = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                if world_match.is_some() && world_match != Some(world) {
                    continue;
                }
                if !stages.contains_key(name) || world_match == Some(world) {
                    stages.insert(name.to_string(), rel.to_string());
                }
            }
        }

        let post: Vec<String> = stages
            .keys()
            .filter(|n| regex::Regex::new(r"^(deferred|composite)\d*$").unwrap().is_match(n))
            .cloned()
            .collect::<Vec<_>>();
        // Sort: deferred before composite, then by number
        let mut post_sorted = post;
        post_sorted.sort_by(|a, b| {
            let group = |n: &str| if n.starts_with("deferred") { 0 } else { 1 };
            let num_a = regex::Regex::new(r"\d+$").unwrap().find(a).and_then(|m| m.as_str().parse::<i32>().ok()).unwrap_or(0);
            let num_b = regex::Regex::new(r"\d+$").unwrap().find(b).and_then(|m| m.as_str().parse::<i32>().ok()).unwrap_or(0);
            group(a).cmp(&group(b)).then(num_a.cmp(&num_b))
        });

        let geometry: Vec<String> = stages
            .keys()
            .filter(|n| n.starts_with("gbuffers_"))
            .cloned()
            .collect::<Vec<_>>();
        let mut geometry_sorted = geometry;
        geometry_sorted.sort();

        let shadow: Vec<String> = stages
            .keys()
            .filter(|n| n.starts_with("shadow"))
            .cloned()
            .collect::<Vec<_>>();
        let mut shadow_sorted = shadow;
        shadow_sorted.sort();

        let files: HashMap<String, String> = stages.into_iter().collect();

        ShaderStagePlan {
            post: post_sorted,
            geometry: geometry_sorted,
            shadow: shadow_sorted,
            files,
        }
    }
}

/// Scene samplers available to post-processing stages.
pub const POST_SCENE_SAMPLERS: &[&str] = &["colortex0", "gcolor", "depthtex0", "noisetex"];

/// Validate post-processing stage outputs (port of `validatePostOutputs`).
pub fn validate_post_outputs(source: &str, name: &str) -> Result<(), String> {
    let re_format = regex::Regex::new(r"const\s+int\s+colortex\d+Format\s*=\s*(\w+)").unwrap();
    for caps in re_format.captures_iter(source) {
        let fmt = &caps[1];
        if !matches!(fmt, "RGBA" | "RGBA8") {
            return Err(format!("{} needs unsupported scene format: {}", name, fmt));
        }
    }

    let re_clear_mipmap = regex::Regex::new(r"const\s+bool\s+colortex\d+(?:Clear\s*=\s*false|MipmapEnabled\s*=\s*true)").unwrap();
    if re_clear_mipmap.is_match(source) {
        return Err(format!("{} needs persistent or mipmapped scene attachments", name));
    }

    let re_outputs = regex::Regex::new(r"\bgl_FragData\s*\[\s*(\d+)\s*\]").unwrap();
    let outputs: Vec<i32> = re_outputs.captures_iter(source).filter_map(|c| c[1].parse().ok()).collect();

    let re_layout = regex::Regex::new(r"layout\s*\([^)]*location\s*=\s*(\d+)[^)]*\)\s*out").unwrap();
    let layout: Vec<i32> = re_layout.captures_iter(source).filter_map(|c| c[1].parse().ok()).collect();

    let re_buffers = regex::Regex::new(r"/\*\s*(?:DRAWBUFFERS|RENDERTARGETS)\s*:\s*([^*]+)\*/").unwrap();
    let buffers: Vec<String> = re_buffers.captures_iter(source).filter_map(|c| c.get(1).map(|m| m.as_str().trim().to_string())).collect();

    if outputs.iter().any(|&n| n != 0) || layout.iter().any(|&n| n != 0) || buffers.iter().any(|s| s != "0") {
        return Err(format!("{} requires multiple or nonzero colour attachments", name));
    }

    Ok(())
}

/// Check if a shader program is enabled (port of `shaderProgramEnabled`).
pub fn shader_program_enabled(properties: &str, dimension: i32, name: &str) -> Result<bool, String> {
    let world = match dimension {
        -1 => "world-1",
        1 => "world1",
        _ => "world0",
    };

    let mut entries = HashMap::new();
    for line in properties.replace("\\r\\n", " ").split('\n') {
        let re = regex::Regex::new(r"^\s*([^#!\s=]+)\s*=\s*(.*)$").unwrap();
        if let Some(caps) = re.captures(line) {
            entries.insert(caps[1].to_string(), caps[2].trim().to_string());
        }
    }

    let key1 = format!("program.{}/{}", world, name);
    let key2 = format!("program.{}.enabled", name);
    let value = entries.get(&key1).or_else(|| entries.get(&key2));

    if let Some(v) = value {
        if v != "true" && v != "false" {
            return Err(format!("{} has an unsupported program-enable expression: {}", name, v));
        }
    }

    // Check for unsupported attachment configurations
    for key in [
        "size.buffer.colortex0",
        &format!("scale.{}", name),
        &format!("flip.{}.colortex0", name),
    ] {
        if entries.contains_key(key) {
            return Err(format!("{} needs unsupported attachment configuration: {}", name, key));
        }
    }

    Ok(value != Some(&"false".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_stage_plan_basic() {
        // Test with a mock zip
    }

    #[test]
    fn validate_post_outputs_rejects_bad_format() {
        let source = "const int colortex0Format = RGB8;";
        assert!(validate_post_outputs(source, "test").is_err());
    }

    #[test]
    fn validate_post_outputs_accepts_rgba8() {
        let source = "const int colortex0Format = RGBA8;";
        assert!(validate_post_outputs(source, "test").is_ok());
    }

    #[test]
    fn shader_program_enabled_true() {
        let props = "program.world0/test.enabled = true";
        assert_eq!(shader_program_enabled(props, 0, "test"), Ok(true));
    }

    #[test]
    fn shader_program_enabled_false() {
        let props = "program.world0/test.enabled = false";
        assert_eq!(shader_program_enabled(props, 0, "test"), Ok(false));
    }

    #[test]
    fn shader_program_enabled_dimension() {
        let props = "program.world-1/test.enabled = true";
        assert_eq!(shader_program_enabled(props, -1, "test"), Ok(true));
    }
}