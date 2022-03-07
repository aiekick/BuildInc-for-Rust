# BuildInc-for-Rust

Rust version of https://github.com/aiekick/BuildInc

## BuildInc

the goal of this tool is to increment a build number in a include file c/c++ for the moment

## Syntax

the syntax is 

```
build_inc rule include_file
```

if there is missing params, the tools will not work and will print a error message

## Rule

the rule syntax is  : max_build_number:max_minor_number

by ex, a rule 1000:10 will do :

```
    if (BuildNumber > 1000) 
    {
        BuildNumber = 0;
    	++MinorNumber;
    }
    if (MinorNumber > 10) 
    {
        MinorNumber = 0;
    	++MajorNumber;
    }
```
   
## Include File Content

the include file will contain theses tokens (by ex) :

```
#pragma once

#define BuildNumber 119
#define MinorNumber 5
#define MajorNumber 0
#define BuildId "0.5.119"
```



