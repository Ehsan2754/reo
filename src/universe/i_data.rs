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
use anyhow::{Context, Result};
use log::trace;
use crate::universe::Universe;

impl Universe {
    /// Set vertex data.
    pub fn data(&mut self, v: u32, d: Data) -> Result<()> {
        self.vertices.get_mut(&v).context(format!("Can't find ν{}", v))?.data = Some(d.clone());
        trace!("#data(ν{}, '{}'): data set", v, d.as_hex());
        Ok(())
    }
}

#[test]
fn sets_simple_data() -> Result<()> {
    let mut uni = Universe::empty();
    let data = 42;
    uni.add(0)?;
    uni.data(0, Data::from_int(data))?;
    assert_eq!(data, uni.dataize("Φ")?.as_int()?);
    assert!(uni.inconsistencies().is_empty());
    Ok(())
}
