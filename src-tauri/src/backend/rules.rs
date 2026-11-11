use crate::{Condition, ListenerData, Rule, SearchType, Selection};
use std::path::Path;

pub(super) fn matches_listener(path: &Path, listener: &ListenerData) -> bool {
    if listener.rules.is_empty() { return false; }
    match listener.selection {
        Selection::Any => listener.rules.iter().any(|rule| matches_rule(path, rule)),
        Selection::All => listener.rules.iter().all(|rule| matches_rule(path, rule)),
    }
}

fn matches_rule(path: &Path, rule: &Rule) -> bool {
    use Condition::*;
    if matches!(rule.search, SearchType::FileSize) {
        let Ok(metadata) = path.metadata() else { return false; };
        if !metadata.is_file() { return false; }
        let target = rule.data.size.1.get_size(rule.data.size.0);
        return match rule.condition { Greater => metadata.len() > target, Less => metadata.len() < target, IsEqual => metadata.len() == target, _ => false };
    }
    if !matches!(rule.condition, Includes | NotIncludes | ExactMatch | IsNot) { return false; }
    // Failed reads and wrong object kinds never match, including negative rules.
    let value = match rule.search {
        SearchType::FileName if path.is_file() => path.file_name().map(|v| v.to_string_lossy().into_owned()),
        SearchType::FolderName if path.is_dir() => path.file_name().map(|v| v.to_string_lossy().into_owned()),
        SearchType::FileExtension if path.is_file() => Some(path.extension().map(|v| v.to_string_lossy().into_owned()).unwrap_or_default()),
        SearchType::FileContent if path.is_file() => std::fs::read_to_string(path).ok(),
        SearchType::PathName => Some(path.to_string_lossy().replace('\\', "/")),
        _ => None,
    };
    let Some(value) = value else { return false; };
    let terms: Vec<_> = rule.data.text.iter().filter(|term| !term.trim().is_empty()).collect();
    if terms.is_empty() { return false; }
    let exact = matches!(rule.condition, ExactMatch | IsNot);
    let normalized = value.to_lowercase();
    let matched = terms.iter().any(|term| {
        let term = if matches!(rule.search, SearchType::PathName) { term.replace('\\', "/") } else { term.to_string() };
        if exact { value == term } else { normalized.contains(&term.to_lowercase()) }
    });
    if matches!(rule.condition, NotIncludes | IsNot) { !matched } else { matched }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Data, UnitOpts};
    fn rule(search: SearchType, condition: Condition, terms: &[&str]) -> Rule { Rule {search, condition, data:Data{text:terms.iter().map(|s|s.to_string()).collect(),size:(3,UnitOpts::B)}} }
    #[test] fn text_matching_and_strict_sizes() {
        let dir=assert_fs::TempDir::new().unwrap(); let path=dir.path().join("Report.TXT"); std::fs::write(&path,"ABC").unwrap();
        assert!(matches_rule(&path,&rule(SearchType::FileName,Condition::Includes,&["report"])));
        assert!(matches_rule(&path,&rule(SearchType::FileContent,Condition::Includes,&["abc","missing"])));
        assert!(!matches_rule(&path,&rule(SearchType::FileContent,Condition::ExactMatch,&["AB"])));
        assert!(matches_rule(&path,&rule(SearchType::FileContent,Condition::ExactMatch,&["ABC"])));
        assert!(!matches_rule(&path,&rule(SearchType::FileSize,Condition::Less,&[])));
        assert!(matches_rule(&path,&rule(SearchType::FileSize,Condition::IsEqual,&[])));
        assert!(!matches_rule(&path,&rule(SearchType::FileName,Condition::NotIncludes,&[])));
        let mut listener=ListenerData::default(); listener.selection=Selection::All;
        assert!(!matches_listener(&path,&listener));
        assert!(!matches_rule(dir.path(),&rule(SearchType::FileName,Condition::NotIncludes,&["missing"])));
    }
}
