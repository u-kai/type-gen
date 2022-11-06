pub struct UKai {
    pub(super) id: Option<i64>,
    pub(super) profile: Option<UKaiProfile>,
}

pub struct UKaiProfile {
    pub(super) follower: Option<Vec<UKaiProfileFollower>>,
    pub(super) name: Option<String>,
    #[serde(rename = "userId")]
    pub(super) user_id: Option<i64>,
}

pub struct UKaiProfileFollower {
    pub(super) name: Option<String>,
}
