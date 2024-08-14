use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dispute {
    pub always_apply: Option<bool>,
    #[serde(flatten)]
    pub dispute: PrefixOrSuffix,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum PrefixOrSuffix {
    Prefix { prefix: String },
    Suffix { suffix: String },
}

impl Dispute {
    pub fn need_prefix(&self) -> Option<&str> {
        self.always_apply
            .unwrap_or_default()
            .then(|| self.dispute.get_prefix())
            .flatten()
    }

    pub fn need_suffix(&self) -> Option<&str> {
        self.always_apply
            .unwrap_or_default()
            .then(|| self.dispute.get_suffix())
            .flatten()
    }
}

impl PrefixOrSuffix {
    fn get_prefix(&self) -> Option<&str> {
        match self {
            Self::Prefix { prefix } => Some(prefix.as_str()),
            Self::Suffix { .. } => None,
        }
    }

    fn get_suffix(&self) -> Option<&str> {
        match self {
            Self::Prefix { .. } => None,
            Self::Suffix { suffix } => Some(suffix.as_str()),
        }
    }

    #[cfg(test)]
    fn prefix(prefix: impl ToString) -> Self {
        let prefix = prefix.to_string();
        Self::Prefix { prefix }
    }

    #[cfg(test)]
    fn suffix(suffix: impl ToString) -> Self {
        let suffix = suffix.to_string();
        Self::Suffix { suffix }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    fn dispute(text: &str) -> json::Result<Dispute> {
        json::from_str(text)
    }

    #[test]
    fn prefix() {
        let dispute = dispute(r#"{"prefix":"SomePrefix"}"#).unwrap();
        assert!(dispute.always_apply.is_none());
        assert_eq!(dispute.dispute, PrefixOrSuffix::prefix("SomePrefix"));
    }

    #[test]
    fn suffix() {
        let dispute = dispute(r#"{"suffix":"SomeSuffix"}"#).unwrap();
        assert!(dispute.always_apply.is_none());
        assert_eq!(dispute.dispute, PrefixOrSuffix::suffix("SomeSuffix"));
    }
}
