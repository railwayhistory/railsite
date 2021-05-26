
use std::collections::HashMap;
use raildata::document::{Document, DocumentLink, LineLink, PointLink};
use raildata::types::{List, Set};
use raildata::library::Library;


//------------ PointConnections ----------------------------------------------

#[derive(Clone, Default)]
pub struct PointConnections {
    lines: HashMap<PointLink, List<LineLink>>,
    connections: HashMap<PointLink, Set<PointLink>>,
}

impl PointConnections {
    pub(super) fn insert(&mut self, document: &Document, link: DocumentLink) {
        if let Some(line) = document.try_as_line() {
            for point in line.points.iter() {
                self.lines.entry(
                    point.into_value()
                ).or_default().push(link.into())
            }
        }
        else if let Some(point) = document.try_as_point() {
            let point_link = link.into();
     
            for event in &point.events {
                if let Some(ref conns) = event.connection {
                    for conn in conns {
                        let conn = conn.into_value();
                        if conn == point_link {
                            continue
                        }
                        self.connections.entry(conn).or_default().insert(
                            point_link
                        );
                        self.connections.entry(point_link).or_default().insert(
                            conn
                        );
                    }
                }
            }
        }
    }

    pub(super) fn finalize(&mut self, library: &Library) {
        // Add lines from connected points, too.
        for (point, links) in &self.connections {
            for link in links.iter() {
                let extra = self.lines.get(link).cloned();
                if let Some(linked_lines) = extra {
                    self.lines.entry(*point).or_default().extend_from_slice(
                        linked_lines.as_slice()
                    )
                }
            }
        }

        // Sort lines by their key.
        for lines in self.lines.values_mut() {
            lines.sort_by(|left, right| {
                let left = left.follow(library);
                let right = right.follow(library);
                left.key().cmp(right.key())
            })
        }
    }

    pub fn get_connections(&self, point: PointLink) -> Option<&Set<PointLink>> {
        self.connections.get(&point)
    }

    pub fn get_lines(&self, point: PointLink) -> &[LineLink] {
        match self.lines.get(&point) {
            Some(res) => res.as_slice(),
            None => &[]
        }
    }

    pub fn is_junction(&self, point: PointLink) -> bool {
        self.get_lines(point).len() > 1
    }
}

