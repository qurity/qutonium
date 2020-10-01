## JSON Format

The json file will be automatically created into a `tmp` directory.

```
{
  "fails":1,
  "name":"the suite case name",
  "successes":5,
  "tests":[
    {
      "column":7,
      "diagnostic":"ok",
      "file":"src/main.rs",
      "line":23,
      "name":"test it should do something 1",
      "pass":true,
      "time_unit":{
        "micros":25,
        "millis":0,
        "nanos":25961,
        "secs":0
      }
    },
    {
      "column":7,
      "diagnostic":"ok",
      "file":"src/main.rs",
      "line":27,
      "name":"test it should do something 2",
      "pass":true,
      "time_unit":{
        "micros":10,
        "millis":0,
        "nanos":10575,
        "secs":0
      }
    },
    {
      "column":7,
      "diagnostic":"ok",
      "file":"src/main.rs",
      "line":31,
      "name":"test it should do something 3",
      "pass":true,
      "time_unit":{
        "micros":1,
        "millis":0,
        "nanos":1197,
        "secs":0
      }
    },
    {
      "column":7,
      "diagnostic":"ok",
      "file":"src/main.rs",
      "line":35,
      "name":"catch closure that should panic",
      "pass":true,
      "time_unit":{
        "micros":100,
        "millis":0,
        "nanos":100601,
        "secs":0
      }
    },
    {
      "column":7,
      "diagnostic":"received: <false> expected: <true>",
      "file":"src/main.rs",
      "line":39,
      "name":"test it should do something 3",
      "pass":false,
      "time_unit":{
        "micros":1,
        "millis":0,
        "nanos":1563,
        "secs":0
      }
    },
    {
      "column":7,
      "diagnostic":"ok",
      "file":"src/main.rs",
      "line":43,
      "name":"test it should do something 3",
      "pass":true,
      "time_unit":{
        "micros":1,
        "millis":0,
        "nanos":1245,
        "secs":0
      }
    }
  ],
  "time_unit_type":"Milliseconds",
  "total":6
}
```