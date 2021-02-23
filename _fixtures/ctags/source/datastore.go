//   tags2uml
//   Copyright 2014 ruben2020 https://github.com/ruben2020/
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

package pkg

type memberinfo_st struct {
	name, access, datatype string
}

type methodinfo_st struct {
	name, access, returntype string
}

type classinfo_st struct {
	name    string
	id      int
	parents []string
	members []memberinfo_st
	methods []methodinfo_st
}

var classmap map[string]classinfo_st
var idcounter int = 1

func InitDatastore() {
	classmap = make(map[string]classinfo_st)
}
