use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct MenuVo {
    pub id: i32,
    #[serde(rename = "title")]
    pub name: String,
    pub r#type: String,
    pub code: Option<String>,
    #[serde(rename = "href")]
    pub url: Option<String>,
    pub open_type: Option<String>,
    pub parent_id: i32,
    pub icon: Option<String>,
    pub sort: i32,
    pub create_at: String,
    pub update_at: String,
    pub enable: i32,
    // title: String,
    // href: String,
    #[sea_orm(skip)]
    children: Option<Vec<MenuVo>>,
    #[sea_orm(skip)]
    pub checked: String,
}

impl MenuVo {
    pub fn to_tree(menus: Vec<MenuVo>) -> Vec<MenuVo> {
        let mut map = std::collections::HashMap::new();
        for menu in menus.clone() {
            if menu.parent_id == 0 {
                map.insert(menu.id, menu);
            }
        }
        for menu in menus {
            if map.contains_key(&menu.parent_id) {
                let parent = map.get_mut(&menu.parent_id).unwrap();
                if parent.children.is_none() {
                    parent.children = Some(vec![]);
                }
                parent.children.as_mut().unwrap().push(menu);
            }
        }
        // order by sort
        for (_, menu) in map.iter_mut() {
            if let Some(children) = &mut menu.children {
                children.sort_by(|a, b| a.sort.cmp(&b.sort));
            }
        }
        let mut roots: Vec<MenuVo> = map.values().cloned().collect();
        // order by sort
        roots.sort_by(|a, b| a.sort.cmp(&b.sort));
        roots
    }
}
