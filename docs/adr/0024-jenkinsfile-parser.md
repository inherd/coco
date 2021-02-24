# 24. Jenkinsfile parser

Date: 2021-02-24

## Status

2021-02-24 proposed

## Context

examples: https://github.com/oliverlockwood/jenkinsfile-idea-plugin/tree/master/src/com/oliverlockwood/plugins/jenkinsfile

```bnf
{
  parserClass="com.oliverlockwood.plugins.jenkinsfile.parser.JenkinsParser"

  extends="com.intellij.extapi.psi.ASTWrapperPsiElement"

  psiClassPrefix="Jenkins"
  psiImplClassSuffix="Impl"
  psiPackage="com.oliverlockwood.plugins.jenkinsfile.psi"
  psiImplPackage="com.oliverlockwood.plugins.jenkinsfile.psi.impl"

  elementTypeHolderClass="com.oliverlockwood.plugins.jenkinsfile.psi.JenkinsTypes"
  elementTypeClass="com.oliverlockwood.plugins.jenkinsfile.psi.JenkinsElementType"
  tokenTypeClass="com.oliverlockwood.plugins.jenkinsfile.psi.JenkinsTokenType"
}

jenkinsFile ::= item_*

private item_ ::= (step|CRLF|COMMENT)

step ::= (STEP_KEY PARAMETER) | STEP_KEY
```

```flex
package com.oliverlockwood.plugins.jenkinsfile;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;
import com.oliverlockwood.plugins.jenkinsfile.psi.JenkinsTypes;
import com.intellij.psi.TokenType;

%%

%class JenkinsLexer
%implements FlexLexer
%unicode
%function advance
%type IElementType
%eof{  return;
%eof}

CRLF= \n|\r|\r\n
WHITESPACE=[\ \t\f]
STEP_NAME="sh" | "parallel"
SQ="'"
SQ_PARAMETER_CHAR=[^\n\r\f\\']
TQ="'''"
//TQ_PARAMETER='(?:.|\n)+
//END_OF_LINE_COMMENT=("#"|"!")[^\r\n]*

%state WAITING_PARAMETERS

%%

//<YYINITIAL> {END_OF_LINE_COMMENT}                           { yybegin(YYINITIAL); return JenkinsTypes.COMMENT; }

<YYINITIAL> {STEP_NAME}                                      { yybegin(WAITING_PARAMETERS); return JenkinsTypes.STEP_KEY; }

//<YYINITIAL> {SEPARATOR}                                     { yybegin(WAITING_VALUE); return JenkinsTypes.SEPARATOR; }

<WAITING_PARAMETERS> {CRLF}({CRLF}|{WHITESPACE})+           { yybegin(YYINITIAL); return TokenType.WHITE_SPACE; }

<WAITING_PARAMETERS> {WHITESPACE}+                          { yybegin(WAITING_PARAMETERS); return TokenType.WHITE_SPACE; }

//<WAITING_PARAMETERS> {TQ}(?:.|\n)+?{TQ}                         { yybegin(YYINITIAL); return JenkinsTypes.PARAMETER; }

<WAITING_PARAMETERS> {SQ}{SQ_PARAMETER_CHAR}+{SQ}           { yybegin(YYINITIAL); return JenkinsTypes.PARAMETER; }

({CRLF}|{WHITESPACE})+                                      { yybegin(YYINITIAL); return TokenType.WHITE_SPACE; }

{WHITESPACE}+                                               { yybegin(YYINITIAL); return TokenType.WHITE_SPACE; }

.                                                           { return TokenType.BAD_CHARACTER; }
```

### Jenkins Declarative Parser https://github.com/rtyler/jdp

Rust: [https://github.com/rtyler/jdp](https://github.com/rtyler/jdp)

Pest file: [https://github.com/rtyler/jdp/blob/main/src/pipeline.pest](https://github.com/rtyler/jdp/blob/main/src/pipeline.pest)

```

```

## Decision

Decision here...

## Consequences

Consequences here...
