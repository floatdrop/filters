//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Bool Filter implementation, so we can insert this in filter construction
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;
use filter::IntoFilter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Bool(bool);

impl Bool {

    pub fn new(b: bool) -> Bool {
        Bool(b)
    }

}

impl From<bool> for Bool {

    fn from(b: bool) -> Bool {
        Bool::new(b)
    }

}

impl IntoFilter<Bool> for bool {
    type IntoFilt = Bool;

    fn into_filter(self) -> Self::IntoFilt {
        Bool::from(self)
    }
}

impl_operators!(Bool, self e { self.0 }, );
