# [Gitana](https://github.com/SOM-Research/Gitana)

Download URL: [https://hal.inria.fr/hal-01187769/document](https://hal.inria.fr/hal-01187769/document)

Git conceptual schema.

```
Repository
- name : String
- / filesDeleted : Int 
- / emptyLines : Int

Reference
- name : String
- inRemote : Boolean

Commit
- sha : String
- message : String
- size : int
- authored_date : Timestamp 
- committed_date : Timestamp

Developer
- name : String
- email : String
- / commitsPerMonth : Int 
- / filesDeleted : Int  

/ LineDetail
- type : String
- lineNumber : Int
- isCommented : Boolean 
- isPartiallyCommented : Boolean 
- isEmpty : Boolean
- content : String

FileModification
- status : String 
- additions : int 
- deletions : int 
- changes : int 
- patch : String

File
- name : String
- extension : String
- getVersion(Date) : String
```
