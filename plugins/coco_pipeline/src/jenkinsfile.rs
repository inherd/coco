use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JenkinsFile {
    pub name: String,
    pub stages: Vec<JenkinsStage>,
    pub post: Vec<PostConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JenkinsStage {
    pub name: String,
    pub jobs: Vec<JenkinsJob>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JenkinsJob {
    pub name: String,
    pub job: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostConfig {
    pub key: String,
    pub value: Vec<JenkinsJob>,
}

#[cfg(test)]
mod tests {
    use jenkinsfile::Jenkinsfile;

    #[test]
    pub fn should_parse_hello_world() {
        let code = r#"pipeline {
    agent { docker 'maven:3.3.3' }
    stages {
        stage('build') {
            steps {
                sh 'mvn --version'
            }
        }
    }
}
        "#;
        let jenkinsfile = Jenkinsfile::from_str(code).unwrap();

        for stage in jenkinsfile.stages {
            println!("stage # {}", stage.name);
            for ins in stage.steps {
                println!("steps # {}", ins);
            }
        }
    }
}
