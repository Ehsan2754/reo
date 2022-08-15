// Copyright (c) 2022 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::data::Data;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use log::trace;

struct Edge {
    from: u32,
    to: u32,
    a: String,
    k: String,
}

impl Edge {
    fn new(from: u32, to: u32, a: String, k: String) -> Edge {
        Edge {
            from, to, a, k
        }
    }
}

pub type Error = String;

pub type Lambda = fn(&mut Universe) -> Result<u32, Error>;

struct Vertex {
    data: Data,
    lambda: Lambda
}

impl Vertex {
    pub fn empty() -> Self {
        Vertex {
            data: Data::empty(),
            lambda: |_| -> Result<u32, Error> { Ok(0) }
        }
    }
}

pub struct Universe {
    vertices: HashMap<u32, Vertex>,
    edges: HashMap<u32, Edge>
}

impl fmt::Debug for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (i, v) in self.vertices.iter() {
            let data = if v.data.is_empty() {
                "".to_string()
            } else {
                format!("Δ ➞ {},", v.data.as_hex())
            };
            lines.push(format!(
                "ν{} -> ⟦{}{}⟧",
                i,
                data.as_str(),
                self.edges
                    .iter()
                    .filter(|(_, e)| e.from == *i)
                    .map(|(j, e)| format!("\n\t{} ε{}➞ ν{}", e.a, j, e.to))
                    .collect::<Vec<String>>()
                    .join("")
            ));
        }
        f.write_str(lines.join("\n").as_str())
    }
}

impl Universe {
    pub fn empty() -> Self {
        Universe {
            vertices: HashMap::new(),
            edges: HashMap::new()
        }
    }

    // Generates the next available ID for vertices and edges.
    pub fn next_id(&mut self) -> u32 {
        let max = self.vertices.keys().max();
        let mut i = 0;
        if let Some(m) = max {
            i = i.max(*m);
        }
        if let Some(k) = self.edges.keys().max() {
            i = i.max(*k)
        }
        trace!("#next_id() -> {}", i);
        i + 1
    }

    // Add a new vertex to the universe.
    pub fn add(&mut self, v: u32) {
        self.vertices.insert(v, Vertex::empty());
        trace!("#add({}): new vertex added", v);
    }

    // Makes an edge `e` from vertex `v1` to vertex `v2` and puts `a` label on it. If the
    // label is not equal to `"ρ"`, makes a backward edge from `v2` to `v1`
    // and labels it as `"ρ"`.
    pub fn bind(&mut self, e: u32, v1: u32, v2: u32, a: &str) {
        self.edges.insert(e, Edge::new(v1, v2, a.to_string(), "".to_string()));
        trace!("#bind({}, {}, {}, \"{}\"): edge added", e, v1, v2, a);
        if a != "ρ" {
            let e1 = self.next_id();
            self.edges.insert(e1, Edge::new(v2, v1, "ρ".to_string(), "".to_string()));
            trace!("#bind({}, {}, {}, \"{}\"): backward ρ-edge added", e, v1, v2, a);
        }
    }

    // Makes an edge `e1` from `v1` to `v2` and puts `a` title and `k` locator on it.
    pub fn reff(&mut self, e1: u32, v1: u32, k: &str, a: &str) {
        let v2 = self.find(v1, k).unwrap();
        self.edges.insert(e1, Edge::new(v1, v2, a.to_string(), k.to_string()));
        trace!("#reff({}, {}, \"{}\", \"{}\"): edge added", e1, v1, k, a);
    }

    // Deletes the edge `e1` and replaces it with a new edge `e2` coming
    // from `v1` to a new vertex `v3`. Also, makes a new edge from `v3` to `v2`.
    pub fn copy(&mut self, e1: u32, v3: u32, e2: u32) {
        let v1 = self.edges.get(&e1).unwrap().from;
        let v2 = self.edges.get(&e1).unwrap().to;
        let a = self.edges.get(&e1).unwrap().a.clone();
        let k = self.edges.get(&e1).unwrap().k.clone();
        self.edges.remove(&e1);
        trace!("#copy({}, {}, {}): edge {} removed", e1, v3, e2, e1);
        self.edges.insert(e2, Edge::new(v1, v3, a.to_string(), k.to_string()));
        trace!("#copy({}, {}, {}): edge {} added", e1, v3, e2, e2);
        let e3 = self.next_id();
        self.edges.insert(e3, Edge::new(v3, v2, "π".to_string(), "".to_string()));
        trace!("#copy({}, {}, {}): π-edge {} added", e1, v3, e2, e3);
    }

    // Set atom lambda.
    pub fn atom(&mut self, v: u32, m: Lambda) {
        self.vertices.get_mut(&v).unwrap().lambda = m;
        trace!("#atom({}, ...): lambda set", v);
    }

    // Set vertex data.
    pub fn data(&mut self, v: u32, d: Data) {
        self.vertices.get_mut(&v).unwrap().data = d.clone();
        trace!("#data({}, \"{}\"): data set", v, d.as_hex());
    }
}

impl Universe {
    // Get one vertex.
    fn vertex(&self, v: u32) -> Option<&Vertex> {
        self.vertices.get(&v)
    }

    // Find a vertex by locator.
    fn find(&mut self, v: u32, loc: &str) -> Result<u32, String> {
        let mut vtx = v;
        loc.split(".").for_each( |k| {
            if k.starts_with("ν") {
                vtx = u32::from_str(&k[2..]).unwrap()
            } else if k == "𝜉" {
                vtx = vtx;
            } else if k == "Φ" {
                vtx = 0;
            } else {
                vtx = self.edges.values().find(
                    |e| e.from == vtx && e.a == k
                ).ok_or(format!("Can't find .{} from ν{}", k, vtx)).unwrap().to
            }
        });
        Ok(vtx)
    }

    // Dataize by locator.
    pub fn dataize(&mut self, v: u32, loc: &str) -> Result<&Data, String> {
        let id = self.find(v, loc)?;
        let v = self.vertex(id).ok_or(format!("ν{} is absent", id))?;
        Ok(&(*v).data)
    }
}

#[cfg(test)]
fn rand(uni: &mut Universe) -> Result<u32, Error> {
    let e = uni.next_id();
    uni.reff(e, 0, "𝜉.int", "i");
    let i = uni.next_id();
    uni.add(i);
    let e2 = uni.next_id();
    uni.copy(e, i, e2);
    let d = uni.next_id();
    uni.add(d);
    let e3 = uni.next_id();
    uni.bind(e3, i, d, "Δ");
    let rnd = rand::random::<i64>();
    uni.data(d, Data::from_int(rnd));
    Ok(i)
}

#[test]
fn generates_unique_ids() {
    let mut uni = Universe::empty();
    let first = uni.next_id();
    assert_eq!(first, uni.next_id());
    uni.add(first);
    assert_ne!(first, uni.next_id());
}

#[test]
fn generates_random_int() {
    let mut uni = Universe::empty();
    uni.add(0);
    let v1 = uni.next_id();
    uni.add(v1);
    let e1 = uni.next_id();
    uni.bind(e1, 0, v1, "int");
    let v2 = uni.next_id();
    uni.add(v2);
    let e2 = uni.next_id();
    uni.bind(e2, 0, v2, "rand");
    let e3 = uni.next_id();
    uni.reff(e3, 0, "ν2", "x");
    uni.atom(v1, rand);
    println!("{uni:?}");
    assert_ne!(
        uni.dataize(0, "x.Δ").unwrap().as_int(),
        uni.dataize(0, "x.Δ").unwrap().as_int()
    );
}
