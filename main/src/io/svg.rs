use super::super::mesh::SharedMesh;

use std::io::BufWriter;
use std::io::prelude::*;
use hashbrown::HashSet;

pub fn write<T: Write>(shared_mesh: &SharedMesh, writer: &mut BufWriter<T>) {

    macro_rules! write {
        () => {{
            writer.write("\n".as_bytes()).unwrap();
        }};
        ($text:expr) => {{
            writer.write($text.as_bytes()).unwrap();
            write!();
        }};
        ($text:expr, $($args:expr), *) => {{
            writer.write(format!($text, $($args), *).as_bytes()).unwrap();
            write!();
        }}
    }

    let positions = &shared_mesh.positions;
    let bbox = crate::base::Box3::new_from_points(positions);

    let (x_bounds, y_bounds) = ((bbox.min.x, bbox.min.y), (bbox.max.x, bbox.max.y));
    let scale = 800.0 / (x_bounds.1 - x_bounds.0).max(y_bounds.1 - y_bounds.0);
    let line_width = 2.0;
    let dx = |x| { scale * (x - x_bounds.0) + line_width};
    let dy = |y| { scale * (y_bounds.1 - y) + line_width};

    let mut edges = HashSet::new();

    let triangles = &shared_mesh.triangles;
    for tri in triangles {
        edges.insert(crate::mesh::Edge::new(tri[0], tri[1]));
        edges.insert(crate::mesh::Edge::new(tri[0], tri[2]));
        edges.insert(crate::mesh::Edge::new(tri[1], tri[2]));
    }

    // Header
    write!(r#"<svg viewbox="auto" xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, scale * (x_bounds.1 - x_bounds.0) + 2.0 * line_width, scale * (y_bounds.1 - y_bounds.0) + 2.0 * line_width);
    write!(r#"<rect x="0" y="0" width="{}" height="{}" style="fill:rgb(0,0,0)" />"#, dx(x_bounds.1) + line_width, dy(y_bounds.0) + line_width);

    for edge in edges {
        let p1 = positions[edge.pos_a as usize];
        let p2 = positions[edge.pos_b as usize];
        write!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:rgb(255,255,255)" stroke-width="1" stroke-linecap="round" />"#, p1.x, p1.y, p2.x, p2.y);
    }

    write!("</svg>");
}