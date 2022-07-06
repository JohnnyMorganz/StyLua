window.BENCHMARK_DATA = {
  "lastUpdate": 1657132677520,
  "repoUrl": "https://github.com/JohnnyMorganz/StyLua",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "distinct": true,
          "id": "8724ea2d302335e73595707c61a6ea4089b7aabf",
          "message": "Cleanup README contents\n\nReduce the verbosity somewhat to make it easier to read",
          "timestamp": "2022-06-26T19:18:44+01:00",
          "tree_id": "d62e80f4d63a26eeefbce0772427d209df8ad2c1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8724ea2d302335e73595707c61a6ea4089b7aabf"
        },
        "date": 1656267896762,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66599457,
            "range": "± 843350",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2429474587,
            "range": "± 6726699",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48883980,
            "range": "± 507335",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "distinct": true,
          "id": "4e9f45432247d9635f5ba0c108fd6f04f5551636",
          "message": "Minor readme cleanup",
          "timestamp": "2022-06-26T19:24:30+01:00",
          "tree_id": "87b6426e7bab4866d42c8ccf16d6db7272f04283",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4e9f45432247d9635f5ba0c108fd6f04f5551636"
        },
        "date": 1656268270692,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 60142578,
            "range": "± 300636",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2736650339,
            "range": "± 2548463",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 52392872,
            "range": "± 215979",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d435cb8bb6a34398cd34d06627b517f5848db836",
          "message": "Hang assignment at equal token before expanding RHS (#342)\n\n* Change around the assignment tactic\r\n- We now try both hanging at equals and normal, and pick the one which uses the least amount of lines\r\n\r\n* Add new test case\r\n\r\n* Fix shape calculation\r\n\r\n* Update some tests\r\n\r\n* Commit other test cases which im not sure im happy about\r\n\r\n* Update new tests\r\n\r\n* Update changelog\r\n\r\n* Fix\r\n\r\n* Update luau tests\r\n\r\n* Prevent hanging at the equals token if the RHS is an if-expr\r\n\r\n* Undo diffs to luau files",
          "timestamp": "2022-06-27T13:26:37+01:00",
          "tree_id": "f14236752a5d9360545568d893ea5a091fd261cc",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d435cb8bb6a34398cd34d06627b517f5848db836"
        },
        "date": 1656333171413,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 63057898,
            "range": "± 452286",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2455471676,
            "range": "± 7916723",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 51411706,
            "range": "± 480763",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "30d5d83479cd5baa20bdda139a6eb1757d5f409c",
          "message": "Fix comment indentation on elseif/else token (#480)\n\n* Add test case\r\n\r\n* Keep comments in line with else(if) token if previous block has contents\r\n\r\n* Update snapshot\r\n\r\n* Update changelog",
          "timestamp": "2022-06-27T14:15:24+01:00",
          "tree_id": "1a41e76f46d4592ca244c087c2d2c16a05b7a5b2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/30d5d83479cd5baa20bdda139a6eb1757d5f409c"
        },
        "date": 1656336128864,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 62841667,
            "range": "± 607150",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2734252229,
            "range": "± 3851964",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 53799086,
            "range": "± 254077",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "distinct": true,
          "id": "0f26d0fbc8aa10f42f96c5883f04c1a05c45e354",
          "message": "Fix large scale comparison",
          "timestamp": "2022-06-27T14:34:50+01:00",
          "tree_id": "484a90c9f1db13fc765df2aac57b4953e680ef59",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0f26d0fbc8aa10f42f96c5883f04c1a05c45e354"
        },
        "date": 1656337427163,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 89028898,
            "range": "± 2075515",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3809358150,
            "range": "± 45809069",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 71780141,
            "range": "± 2045530",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "distinct": true,
          "id": "c9f00015e7fe60e7457a760dd89e3190305859c5",
          "message": "Re-include zombie strike",
          "timestamp": "2022-06-27T14:40:28+01:00",
          "tree_id": "c89586ac066fc2207b24c9a4b596a0fe325817dc",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c9f00015e7fe60e7457a760dd89e3190305859c5"
        },
        "date": 1656337844462,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 64696634,
            "range": "± 729839",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2747420557,
            "range": "± 4911025",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 53937522,
            "range": "± 607852",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "86d45b7403172e5c03b72866d01ce41b920fac30",
          "message": "Workflow testing (#481)\n\n* Update release\r\n\r\n* trigger on dispatch\r\n\r\n* fix",
          "timestamp": "2022-06-27T16:25:00+01:00",
          "tree_id": "ea4355437fdaff3a26c7a661ad32a75152e9b62c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/86d45b7403172e5c03b72866d01ce41b920fac30"
        },
        "date": 1656343895749,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 63449750,
            "range": "± 536252",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2733690205,
            "range": "± 2901885",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 53498074,
            "range": "± 488814",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "distinct": true,
          "id": "59227db06a59b23d50697816f5f94f1d03d6f9ca",
          "message": "Fix workflow issues",
          "timestamp": "2022-06-27T16:28:38+01:00",
          "tree_id": "4a8b70846f0f585d513b59cf61af57cf2d9e12da",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/59227db06a59b23d50697816f5f94f1d03d6f9ca"
        },
        "date": 1656344258361,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 90383561,
            "range": "± 3344268",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3921860189,
            "range": "± 47514702",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 74236737,
            "range": "± 2686480",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1c9f775ef64da8376966aade97c728d1c9267490",
          "message": "Expose `format_ast` functionality (#483)\n\n* Expose `format_ast` functionality\r\n\r\n* Fix documentation\r\n\r\n* Retrigger workflow",
          "timestamp": "2022-06-27T18:40:24+01:00",
          "tree_id": "b70caf0cab0c2f6384b2098d25c29ccbbce88f21",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1c9f775ef64da8376966aade97c728d1c9267490"
        },
        "date": 1656352077153,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 78422345,
            "range": "± 682756",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3155737001,
            "range": "± 157552687",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 62198508,
            "range": "± 2817529",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "35d5bea7c7efa6d0d84302384aa5884787586920",
          "message": "Hang static chained function calls (#470)\n\n* Hang static chained function calls\r\n\r\n* Inline first call in chain depending on heuristics (#476)\r\n\r\n* Inline first chain call using heuristics\r\n\r\n* Keep chain inlined if the first call is inlined and there is only 2 indexes\r\n\r\n* Expand call chain if inlined version goes over width",
          "timestamp": "2022-07-05T20:47:04+01:00",
          "tree_id": "89be6523054b6af57ff95e66afea47558f4146e9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/35d5bea7c7efa6d0d84302384aa5884787586920"
        },
        "date": 1657050887071,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 75455019,
            "range": "± 1334746",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3153274524,
            "range": "± 25376528",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 65888386,
            "range": "± 627572",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "johnnymorganz@outlook.com",
            "name": "JohnnyMorganz",
            "username": "JohnnyMorganz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "69c9278e551be7681578e01e6ab16cf6b05c82c5",
          "message": "Use initial comment indentation level for elseif/else comments (#488)\n\n* Use input formatting to determine indent level of elseif/else comments\r\n\r\n* Update changelog\r\n\r\n* Add another test case\r\n\r\n* Fix snapshot\r\n\r\n* Change code",
          "timestamp": "2022-07-06T19:32:23+01:00",
          "tree_id": "05a756ced5eca858dfeb369f5b2c04135687604b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/69c9278e551be7681578e01e6ab16cf6b05c82c5"
        },
        "date": 1657132676933,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58008260,
            "range": "± 604617",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2427619407,
            "range": "± 2563436",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48942147,
            "range": "± 330437",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}