# nu_plugin_formats
A nushell plugin to convert data to nushell tables.

# support formats:
1. from eml - original ported from nushell core.
2. from ics - original ported from nushell core.
3. from ini - original ported from nushell core.
4. from vcf - original ported from nushell core.

# Examples
## from eml
1. Convert eml structured data into record
```
> 'From: test@email.com
Subject: Welcome
To: someone@somewhere.com

Test' | from eml
```

2. Convert eml structured data into record with restricted body to view
```
> 'From: test@email.com
Subject: Welcome
To: someone@somewhere.com

Test' | from eml -b 1
```

## from ics
Converts ics formatted string to table
```
> 'BEGIN:VCALENDAR
END:VCALENDAR' | from ics
```

## from vcf
Converts ics formatted string to table
```
> 'BEGIN:VCARD
N:Foo
FN:Bar
EMAIL:foo@bar.com
END:VCARD' | from vcf
```

## from ini
Converts ini formatted string to record
```
> '[foo]
a=1
b=2' | from ini
```

# Note
Currently to run tests successfully, you need to put a binary `nu` file into `target/debug/`.  It's no-longer required if https://github.com/nushell/nushell/pull/7942 is ok to merge.
