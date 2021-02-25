# Coco Pipeline Analyser

https://docs.github.com/cn/actions/learn-github-actions/migrating-from-jenkins-to-github-actions

```schedule
pipeline {
  agent any
  triggers {
    cron('H/15 * * * 1-5')
  }
}
```

examples:

```groovy
pipeline {
  agent none
  stages {
    stage('Run Tests') {
      matrix {
        axes {
          axis {
            name: 'PLATFORM'
            values: 'macos', 'linux'
          }
        }
        agent { label "${PLATFORM}" }
        stages {
          stage('test') {
            tools { nodejs "node-12" }
            steps {
              dir("scripts/myapp") {
                sh(script: "npm install -g bats")
                sh(script: "bats tests")
              }
            }
          }
        }
      }
    }
  }
}
```
