pub mod catalog {
    pub mod category {
    
    }

    pub mod brand {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct Brand {
            #[serde(rename = "Name")]
            name: String,
            #[serde(rename = "Text")]
            text: Option<String>,
            #[serde(rename = "Keywords")]
            keywords: Option<String>,
            #[serde(rename = "SiteTitle")]
            site_title: Option<String>,
            #[serde(rename = "Active")]
            active: bool,
            #[serde(rename = "MenuHome")]
            menu_home: Option<String>,
            #[serde(rename = "AdWordsRemarketingCode")]
            ad_words_remarketing_code: Option<String>,
            #[serde(rename = "LomadeeCampaignCode")]
            lomadee_campaign_code: Option<String>,
            #[serde(rename = "Score")]
            score: Option<i32>,
        }

        impl Brand {
            pub fn build_category(
                name: String,
                text: Option<String>,
                keywords: Option<String>,
                site_title: Option<String>,
                active: bool,
                menu_home: Option<String>,
                ad_words_remarketing_code: Option<String>,
                lomadee_campaign_code: Option<String>,
                score: Option<i32>
            ) -> Brand {
                Brand {
                    name,
                    text,
                    keywords,
                    site_title,
                    active,
                    menu_home,
                    ad_words_remarketing_code,
                    lomadee_campaign_code,
                    score,
                }
            }
            
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
