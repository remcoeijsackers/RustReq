#[derive(Clone)]
pub struct reqobj {
    pub rpattern: String,
    pub rtype: String,
}

pub fn build_reqobj(rpattern: String, rtype: String) -> reqobj {
    reqobj {
        rpattern : rpattern,
        rtype : rtype,
    }
}

