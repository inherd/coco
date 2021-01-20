use crate::domain::cloc::{ClocDetail, ClocLanguage};
use crate::infrastructure::cloc;
use std::path::PathBuf;

pub fn analysis(path: PathBuf) -> Vec<ClocLanguage> {
    let mut languages = vec![];
    for (lang_type, language) in cloc::by_dir(path) {
        let mut details = vec![];
        for report in language.reports {
            details.push(ClocDetail {
                blanks: report.stats.blanks,
                code: report.stats.code,
                comments: report.stats.comments,
                name: report.name.to_str().unwrap().to_string(),
            });
        }

        languages.push(ClocLanguage {
            language: lang_type.to_string(),
            blanks: language.blanks,
            code: language.code,
            comments: language.comments,
            reports: details,
        })
    }

    return languages;
}

#[cfg(test)]
mod test {
    use crate::app::cloc_analysis;
    use std::path::PathBuf;

    fn fixtures_dir() -> PathBuf {
        return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("_fixtures");
    }

    #[test]
    fn should_cloc_in_dir() {
        let buf = fixtures_dir().join("projects").join("java").join("hello");
        let languages = cloc_analysis::analysis(buf);

        assert_eq!(1, languages.len());
        assert_eq!("Java", languages[0].language);
        assert_eq!(1, languages[0].blanks);
        assert_eq!(6, languages[0].code);
        assert_eq!(1, languages[0].reports.len());
        assert_eq!(1, languages[0].reports[0].blanks);
        assert_eq!(6, languages[0].reports[0].code);
    }
}
