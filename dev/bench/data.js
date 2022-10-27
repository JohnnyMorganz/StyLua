window.BENCHMARK_DATA = {
  "lastUpdate": 1666872913271,
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
          "id": "4f956b99bd422f805df0ad7162473b44369f3088",
          "message": "Simplify \"simple heuristics\" even further (#492)",
          "timestamp": "2022-07-06T20:23:47+01:00",
          "tree_id": "07a65b0d08b2af43b68e5bf1268c5a67595199a7",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4f956b99bd422f805df0ad7162473b44369f3088"
        },
        "date": 1657135774243,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 62342713,
            "range": "± 430800",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2397233025,
            "range": "± 6665575",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 52797098,
            "range": "± 374369",
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
          "id": "2be84259573cdc79ac51d4cb76c7ae26ff1db73b",
          "message": "Prevent hanging on equals token for complex expression (#491)\n\n* Don't hang on complex function calls\r\n\r\n* Add test\r\n\r\n* Update changelog\r\n\r\n* Fix\r\n\r\n* Add another test case",
          "timestamp": "2022-07-06T20:25:12+01:00",
          "tree_id": "ed238a035b1f564fc1cf8b3aa330d996b65cece9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2be84259573cdc79ac51d4cb76c7ae26ff1db73b"
        },
        "date": 1657135865190,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65906600,
            "range": "± 1343062",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2408900537,
            "range": "± 3740364",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 55121011,
            "range": "± 463699",
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
          "id": "88a95221a35c6415b1807e43a3d898832fc9ea57",
          "message": "Collapse simple statements onto single line, behind option (#479)\n\n* Update trivia formatters for LastStmt\r\n\r\n* Separate laststmt formatting from trivia addition\r\n\r\n* Add configuration for collapse mode\r\n\r\n* Add test case\r\n\r\n* Update function formatting for singleline mode\r\n\r\n* Keep expanded if return is complex / multiline + add tests\r\n\r\n* Rename to collapse simple statement\r\n\r\n* Add support for collapsing if guards\r\n\r\n* Fix indentation of collapsed if statements\r\n\r\n* Prevent collapsing nested functions\r\n\r\n* Undo call expansion change\r\n\r\n* Fix should expand parens check\r\n\r\n* Update tests\r\n\r\n* Add test for long conditional\r\n\r\n* Add more test cases for nested functions\r\n\r\n* More nested function tests\r\n\r\n* Fix coverage\r\n\r\n* Try improve code coverage\r\n\r\n* Update changelog\r\n\r\n* Fix bug\r\n\r\n* Allow collapsing functions with simple stmts as well\r\n\r\ne.g. an assignment or function call\r\n\r\n* Rustfmt\r\n\r\n* Also do for if statements\r\n\r\n* Fix function body shape resetting\r\n\r\n* Mark block as not simple if its a multiple assignment\r\n\r\n* Add test cases\r\n\r\n* Fix bug\r\n\r\n* Fix luau test\r\n\r\n* Fix\r\n\r\n* More test cases",
          "timestamp": "2022-07-06T20:57:39+01:00",
          "tree_id": "9ab2b4c7da295ad9b8e5b8cd5083ff79519b4877",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/88a95221a35c6415b1807e43a3d898832fc9ea57"
        },
        "date": 1657137965820,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 108160119,
            "range": "± 4420807",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3754853818,
            "range": "± 86081647",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 80649286,
            "range": "± 3213480",
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
          "id": "521d979ae29474186d5cc17005915755299c84e4",
          "message": "v0.14.0",
          "timestamp": "2022-07-06T22:18:04+01:00",
          "tree_id": "02282b729ed326407b98aa3d61961a0e02f66ec8",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/521d979ae29474186d5cc17005915755299c84e4"
        },
        "date": 1657142790410,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67584383,
            "range": "± 769442",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2522617011,
            "range": "± 3756713",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 55219725,
            "range": "± 246113",
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
          "id": "44f531b04e50410707a4ce73ceb590f77d7d6cff",
          "message": "Fix build wasm script",
          "timestamp": "2022-07-06T22:22:31+01:00",
          "tree_id": "ac4ad17545b033f0f03fecd03e6d2d1eeed3d276",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/44f531b04e50410707a4ce73ceb590f77d7d6cff"
        },
        "date": 1657143215586,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67649748,
            "range": "± 823906",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2516114697,
            "range": "± 4115051",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54628326,
            "range": "± 248789",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "47758296+Wyatt-Stanke@users.noreply.github.com",
            "name": "Wyatt Stanke",
            "username": "Wyatt-Stanke"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eb29c03eec0f7ef81e38bac03ea1735b0b62fee8",
          "message": "Fix README typo (#498)",
          "timestamp": "2022-07-17T23:52:15+01:00",
          "tree_id": "63866e3c05e916277f73b07b7c06e50838d54d6e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/eb29c03eec0f7ef81e38bac03ea1735b0b62fee8"
        },
        "date": 1658098677845,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 72220998,
            "range": "± 914145",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2508746135,
            "range": "± 3007967",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 57050964,
            "range": "± 360337",
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
          "id": "f386cf1e24249934a25b3e57acd814c1b60d8d71",
          "message": "Fix clippy warnings (#501)\n\n* Fix clippy warnings\r\n\r\n* Fix\r\n\r\n* Revert",
          "timestamp": "2022-07-20T20:41:35+01:00",
          "tree_id": "1e8e8823e9fcefb23d7c02f2dbece6bd399c39e4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f386cf1e24249934a25b3e57acd814c1b60d8d71"
        },
        "date": 1658346438104,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66177955,
            "range": "± 683374",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2497054843,
            "range": "± 5526838",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54537721,
            "range": "± 525173",
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
          "id": "fe8c8e338133665eabf1a20f78c7a977b77bad6f",
          "message": "Fix var expression collapsing when containing comments (#502)\n\n* Add test\r\n\r\n* Repurpose function call formatting for var expression\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2022-07-20T20:46:27+01:00",
          "tree_id": "b3ff03a5e23f44ebbc04c32ea18fd53692c18391",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fe8c8e338133665eabf1a20f78c7a977b77bad6f"
        },
        "date": 1658346730432,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66009219,
            "range": "± 991687",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2498998247,
            "range": "± 5405296",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54040035,
            "range": "± 218811",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "hello@muniftanjim.dev",
            "name": "Munif Tanjim",
            "username": "MunifTanjim"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bd577e0fee4e10f81b79f4663d1154fad62dbd09",
          "message": "Fix ignore behavior for --stdin-filepath (#495)",
          "timestamp": "2022-07-21T15:03:29+01:00",
          "tree_id": "caf09e3b53b7b45127822cef9fbf6c254521be41",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bd577e0fee4e10f81b79f4663d1154fad62dbd09"
        },
        "date": 1658412554620,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67179851,
            "range": "± 800280",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2511918928,
            "range": "± 4916914",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54614363,
            "range": "± 226864",
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
          "id": "71bbf4ea300523fc601cad48b6e85a6ea970477e",
          "message": "v0.14.1",
          "timestamp": "2022-07-21T22:24:01+01:00",
          "tree_id": "bd75c30b5404253de366a5962ed5850bbf0d096d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/71bbf4ea300523fc601cad48b6e85a6ea970477e"
        },
        "date": 1658439076918,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 83709105,
            "range": "± 939223",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2975657782,
            "range": "± 10023568",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 67511339,
            "range": "± 600539",
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
          "id": "7672adf66be99178d9f97bd0413e5cbc9e334c15",
          "message": "Fix wasm",
          "timestamp": "2022-07-22T19:03:06+01:00",
          "tree_id": "d88fd3c76d0430b40bdbc4df069987f21bb02d66",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7672adf66be99178d9f97bd0413e5cbc9e334c15"
        },
        "date": 1658513366723,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65624839,
            "range": "± 359842",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2521302835,
            "range": "± 15469134",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54340199,
            "range": "± 312757",
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
          "id": "4f8afdf81d8e6460a819f46f3d2aba2cddcdc97f",
          "message": "Set content type of release assets to zip (#510)",
          "timestamp": "2022-07-26T19:15:55+01:00",
          "tree_id": "04b12cd08224dd27f1fb05f94624b35c5df37b70",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4f8afdf81d8e6460a819f46f3d2aba2cddcdc97f"
        },
        "date": 1658859811102,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 90209329,
            "range": "± 2446164",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3473539399,
            "range": "± 132338469",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 75254697,
            "range": "± 1993792",
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
          "id": "51fdff45eac4a2a060e76eb8176774df0765eda5",
          "message": "Fix collapsing when varexpr prefix has trailing comments (#511)\n\n* Add test case\r\n\r\n* Check for trailing comments on varexpr prefix for hang\r\n\r\n* Update snapshot\r\n\r\n* Update changelog",
          "timestamp": "2022-07-26T19:40:29+01:00",
          "tree_id": "1e01aa5fe44525b8d896491455f5610dbe7208df",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/51fdff45eac4a2a060e76eb8176774df0765eda5"
        },
        "date": 1658861160253,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 70157703,
            "range": "± 784279",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2573337209,
            "range": "± 105011280",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 59361649,
            "range": "± 1395652",
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
          "id": "ec67af42d0a154e8c23c4bd1ecaebdd6a4f5487b",
          "message": "Fix collapsing when comment between return and expr (#513)\n\n* Add snapshot\r\n\r\n* Handle comments between return and expr\r\n\r\n* Fix eager comment check\r\n\r\n* Update snapshot\r\n\r\n* Update changelog\r\n\r\n* Fix snap",
          "timestamp": "2022-07-27T21:43:25+01:00",
          "tree_id": "0aed7a86c065c934ba4e03f90e3fb221726eee99",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ec67af42d0a154e8c23c4bd1ecaebdd6a4f5487b"
        },
        "date": 1658954932778,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 73002263,
            "range": "± 574608",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2258526387,
            "range": "± 3533810",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 52380252,
            "range": "± 472132",
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
          "id": "0ae6a84cfcd27a03e38d3cb39cbb8e05aa011df9",
          "message": "v0.14.2",
          "timestamp": "2022-07-27T21:51:57+01:00",
          "tree_id": "2388b28cadf0a911223d83563d3cb51ddb5ee751",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0ae6a84cfcd27a03e38d3cb39cbb8e05aa011df9"
        },
        "date": 1658955473105,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65754462,
            "range": "± 1483633",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2519631797,
            "range": "± 3178892",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54227889,
            "range": "± 447893",
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
          "id": "86fae140327ed92f7b975283d75632740c34f04a",
          "message": "Expand regression test suite (#523)\n\n* Expand regression suite\r\n\r\n* Limit folder\r\n\r\n* Handle multi arg commands",
          "timestamp": "2022-08-07T13:56:42+01:00",
          "tree_id": "66589b4a4e1ad2d5badc751ee9b0979d70133513",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/86fae140327ed92f7b975283d75632740c34f04a"
        },
        "date": 1659877400423,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 78728298,
            "range": "± 843250",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2904264889,
            "range": "± 19959233",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 66552978,
            "range": "± 640388",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "mvllow@icloud.com",
            "name": "not",
            "username": "mvllow"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f4706e809bbf6a916711393d9b5780969ba61d72",
          "message": "Fix aarch64 target (#529)",
          "timestamp": "2022-08-08T23:25:43+01:00",
          "tree_id": "67a3654c4c239d1069e34213688e7555730d6706",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f4706e809bbf6a916711393d9b5780969ba61d72"
        },
        "date": 1659997905089,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 72474913,
            "range": "± 432781",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2540034257,
            "range": "± 8632534",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 57197811,
            "range": "± 256966",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "last_talon@new.rr.com",
            "name": "Lucas Gangstad",
            "username": "LastTalon"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "11ba9f826ae3cdda00da16dbc76d339be6dfee43",
          "message": "Add changelog links (#532)",
          "timestamp": "2022-08-20T15:15:58+01:00",
          "tree_id": "f53cd97fa4c69447df7269da7fa2c56e45801805",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/11ba9f826ae3cdda00da16dbc76d339be6dfee43"
        },
        "date": 1661005302328,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65300266,
            "range": "± 527398",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2521754695,
            "range": "± 3985582",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 55352343,
            "range": "± 196829",
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
          "id": "0f3dad8abec979dfa07f56c37763d48dbb01944e",
          "message": "Format type parentheses multiline if long union/intersection (#536)\n\n* Add test case\r\n\r\n* Format parentheses multiline if long\r\n\r\n* Snapshot\r\n\r\n* Changelog",
          "timestamp": "2022-08-20T15:22:04+01:00",
          "tree_id": "da5a91047468cb9edf7337ddcf4f375325b30c57",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0f3dad8abec979dfa07f56c37763d48dbb01944e"
        },
        "date": 1661005725993,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 75370798,
            "range": "± 1891311",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2763271567,
            "range": "± 40265865",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 62739991,
            "range": "± 2345601",
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
          "id": "6cb6848325b854ee96706f2e21c6fe0ac999df7b",
          "message": "Keep leading comments when removing excess parentheses (#537)\n\n* Add test case\r\n\r\n* Keep leading comments when removing excess parens\r\n\r\n* Snapshot\r\n\r\n* Changelog\r\n\r\n* Add another test case",
          "timestamp": "2022-08-20T15:51:21+01:00",
          "tree_id": "cbc3d7fdfc128756f1c564fba2fd5fb41094a2cd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6cb6848325b854ee96706f2e21c6fe0ac999df7b"
        },
        "date": 1661007486843,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 79774180,
            "range": "± 717094",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2914906984,
            "range": "± 10633703",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 66135673,
            "range": "± 480669",
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
          "id": "d6bb9c784c5ae175451a9225d72d0506675845de",
          "message": "Fix collapsing when comment present in complex expr (#538)\n\n* Add test case\r\n\r\n* If expression contains comments, then use hanging version\r\n\r\n* Snapshot\r\n\r\n* changelog",
          "timestamp": "2022-08-20T16:04:05+01:00",
          "tree_id": "6be256e6820d324864ebbddf9e9633050d3dcda6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d6bb9c784c5ae175451a9225d72d0506675845de"
        },
        "date": 1661008271208,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57751780,
            "range": "± 5597639",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2168386852,
            "range": "± 13050002",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 49228228,
            "range": "± 6099793",
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
          "id": "a8dd59e5dcba43f1a1096be3e3480840c66602e4",
          "message": "Remove unnecessary else break in if expression comments (#539)\n\n* Add test case\r\n\r\n* Don't break on else if it is not necessary\r\n\r\n* Snapshot\r\n\r\n* Changelog",
          "timestamp": "2022-08-20T16:30:48+01:00",
          "tree_id": "61840e1c492468cf4ef5ff4bd5db64e26751973b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a8dd59e5dcba43f1a1096be3e3480840c66602e4"
        },
        "date": 1661009863150,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 84516975,
            "range": "± 648287",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2969678807,
            "range": "± 7782039",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 68051773,
            "range": "± 1328325",
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
          "id": "c9d2b35e08d9f4df79f158b809649f17b9b0862d",
          "message": "Create `@johnnymorganz/stylua-bin` installable through npm (#540)\n\n* Create binary installable through npm\r\n\r\n* Add workflow to publish npm bin\r\n\r\n* Update README\r\n\r\n* Setup readme in workflow",
          "timestamp": "2022-08-20T18:03:11+01:00",
          "tree_id": "f176dfaacd0886147156f40ea2064f867d3e6edd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c9d2b35e08d9f4df79f158b809649f17b9b0862d"
        },
        "date": 1661015341449,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 68939026,
            "range": "± 728970",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2535572815,
            "range": "± 2678984",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 56328803,
            "range": "± 267301",
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
          "id": "4e9fed49fd0aeceb721954bc2bf572e733bfb9ca",
          "message": "Rename branch",
          "timestamp": "2022-08-21T11:44:12+01:00",
          "tree_id": "d92815a49197f4c0421225a62d9904ca4c81fbb2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4e9fed49fd0aeceb721954bc2bf572e733bfb9ca"
        },
        "date": 1661079134873,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 97995641,
            "range": "± 6260621",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3437880153,
            "range": "± 70607525",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 72721435,
            "range": "± 4006719",
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
          "id": "948f1787fb3a9c825786dfc1b72159a79903eaba",
          "message": "Take into account extra line when hanging assignment (#544)\n\n* Test case\r\n\r\n* Take into account extra line\r\n\r\n* Changelog\r\n\r\n* Snapshot",
          "timestamp": "2022-08-21T13:09:38+01:00",
          "tree_id": "e1958a1059afd14d18f2470f267e111bc5e05dc6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/948f1787fb3a9c825786dfc1b72159a79903eaba"
        },
        "date": 1661084160649,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71628859,
            "range": "± 2996282",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2644905434,
            "range": "± 55129162",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 58027912,
            "range": "± 2268044",
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
          "id": "673643f6990fef82cf6fb8c9d75f6c7b7d57520c",
          "message": "Temporarily disable BlizzardInterfaceCode in LST (#546)\n\n* Empty commit\r\n\r\n* Temporarily disable BlizzardInterfaceCode",
          "timestamp": "2022-08-21T13:37:31+01:00",
          "tree_id": "90f97dd309999ffad907cfde6282ad5eb614044e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/673643f6990fef82cf6fb8c9d75f6c7b7d57520c"
        },
        "date": 1661085800739,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65824301,
            "range": "± 649065",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2514963486,
            "range": "± 6076973",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 55105159,
            "range": "± 155194",
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
          "id": "096bbeb763b789026f5fbbb0a3c182c5698174e9",
          "message": "Format comments added to new trailing comma (#548)\n\n* Add test case\r\n\r\n* Format trailing trivia added to new trailing comma\r\n\r\n* Update changelog\r\n\r\n* Snapshot",
          "timestamp": "2022-08-21T14:13:08+01:00",
          "tree_id": "abfefdd9c3e9c818b2bc61f6469163d15446d57a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/096bbeb763b789026f5fbbb0a3c182c5698174e9"
        },
        "date": 1661087944568,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 68428951,
            "range": "± 475425",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2539044210,
            "range": "± 6940406",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 56450956,
            "range": "± 267784",
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
          "id": "934393db28fda5415ddfd6903be302b0ffc341c5",
          "message": "Keep small prefix inlined in call chain (#550)\n\n* Add test case\r\n\r\n* Ensure small prefix is kept inlined\r\n\r\n* Update changelog\r\n\r\n* Update snapshots",
          "timestamp": "2022-08-21T15:03:33+01:00",
          "tree_id": "e391a4f290c6238e78513c2c0d3bfdc9b32319a6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/934393db28fda5415ddfd6903be302b0ffc341c5"
        },
        "date": 1661091036420,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 80854840,
            "range": "± 1038544",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2961419328,
            "range": "± 15233730",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 66653931,
            "range": "± 598380",
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
          "id": "78ec64f82a764c43fc74cfa79e5860553d83e45e",
          "message": "Fix table field shape calculation (#552)\n\n* Add test case\r\n\r\n* Fix table shape computation\r\n\r\n* Changelog\r\n\r\n* Snapshots",
          "timestamp": "2022-08-21T15:14:11+01:00",
          "tree_id": "1de2ec12281ccc2e0c95e2a33152a99cd4c7c854",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/78ec64f82a764c43fc74cfa79e5860553d83e45e"
        },
        "date": 1661091601720,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66556626,
            "range": "± 420423",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 2501444583,
            "range": "± 4531467",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54461952,
            "range": "± 147958",
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
          "id": "760737fbae0aa62b89b32b405aed4b7c3d0c2976",
          "message": "Don't attempt to hang a prefix string unnecessarily (#545)\n\n* Add test case\r\n\r\n* Don't hang prefix string as it provides no benefit\r\n\r\n* Update changelog\r\n\r\n* Snapshot\r\n\r\n* Fix luau",
          "timestamp": "2022-08-21T19:48:56+01:00",
          "tree_id": "17f8d98d609f686b9d5a3faeeb8e7733149ea413",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/760737fbae0aa62b89b32b405aed4b7c3d0c2976"
        },
        "date": 1661108170470,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 83117596,
            "range": "± 2355676",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 3269076394,
            "range": "± 41683099",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 68724342,
            "range": "± 2727080",
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
          "id": "c91e97e7c904e3c79aec4cebc444e2e04a1df904",
          "message": "Prefer hanging table field value over expanding (#553)\n\n* Hang table field value instead of expanding it\r\n\r\n* Changelog\r\n\r\n* Snapshot\r\n\r\n* Rethink strategy to hang table field value",
          "timestamp": "2022-08-27T10:30:19+01:00",
          "tree_id": "9a7948dfe12af6e8ef85c313d6d94b616b236a00",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c91e97e7c904e3c79aec4cebc444e2e04a1df904"
        },
        "date": 1661592950618,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 78295704,
            "range": "± 3746028",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 557838431,
            "range": "± 29480299",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 50925822,
            "range": "± 1858701",
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
          "id": "834f632f67af6425e7773eaade8d23a880946843",
          "message": "v0.14.3 - fix lockfile and workflow",
          "timestamp": "2022-08-27T11:50:39+01:00",
          "tree_id": "052f995389c4484269558abbb17026ba7190a357",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/834f632f67af6425e7773eaade8d23a880946843"
        },
        "date": 1661597824767,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 93773547,
            "range": "± 2232107",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 615548371,
            "range": "± 17365141",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 56100653,
            "range": "± 2071786",
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
          "id": "52a51ced2ea8e026db94519ff6ddb8bf3a4c7a69",
          "message": "Enforce locked on cargo publish (#557)\n\nEnforce locked on publish",
          "timestamp": "2022-08-27T20:44:01+01:00",
          "tree_id": "de34b2e45894b6e66643bb52f41560997e194943",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/52a51ced2ea8e026db94519ff6ddb8bf3a4c7a69"
        },
        "date": 1661629709570,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67546993,
            "range": "± 865290",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 444440692,
            "range": "± 3330055",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 41461375,
            "range": "± 257289",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ian@iamthefij.com",
            "name": "Ian Fijolek",
            "username": "IamTheFij"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f5243a1ff2f2a0dd59589706d7dccac03a4128f4",
          "message": "Update pyproject to support installing on M1 machines (#558)",
          "timestamp": "2022-08-31T22:15:38+01:00",
          "tree_id": "3bfdd0b8a4471346c757064f38c9147653dfd352",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f5243a1ff2f2a0dd59589706d7dccac03a4128f4"
        },
        "date": 1661980892619,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 93669497,
            "range": "± 3280631",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 589043179,
            "range": "± 18421564",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 55967047,
            "range": "± 2732621",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "greg@hurrell.net",
            "name": "wincent",
            "username": "wincent"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1ac04240fd934b64d6ed31fdf88101228bccaad9",
          "message": "feat: add `--allow-hidden` option (#563)",
          "timestamp": "2022-09-03T10:32:49+01:00",
          "tree_id": "92cfc156e389c7135a89774effccff6ed536eb8f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1ac04240fd934b64d6ed31fdf88101228bccaad9"
        },
        "date": 1662197844321,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65132077,
            "range": "± 555792",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 428713894,
            "range": "± 2977630",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39617655,
            "range": "± 121849",
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
          "id": "82b9df51a898a3bf757e041748e87c0d866e556f",
          "message": "Format changelog",
          "timestamp": "2022-09-03T11:54:50+01:00",
          "tree_id": "eb2c6e72f283b5e3766328115a6684a1e23c960e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/82b9df51a898a3bf757e041748e87c0d866e556f"
        },
        "date": 1662202756807,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 64931473,
            "range": "± 724245",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 433797601,
            "range": "± 1075175",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 40759602,
            "range": "± 101842",
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
          "id": "fa59c036563baf314312509913daa66d9d003a4d",
          "message": "Fix changelog grammar",
          "timestamp": "2022-09-03T11:55:30+01:00",
          "tree_id": "db860da6155a8603136c6a2e4f5d9a2a1c1efa66",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fa59c036563baf314312509913daa66d9d003a4d"
        },
        "date": 1662202795992,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65366680,
            "range": "± 389317",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 428532215,
            "range": "± 2859846",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39834176,
            "range": "± 94541",
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
          "id": "2e3d0488adc81b49ef9aaebb0b3659035f1b3dec",
          "message": "Switch to ubuntu-latest runners for everything except release\n\nNeed to investigate release glibc ubuntu problems",
          "timestamp": "2022-09-03T12:24:10+01:00",
          "tree_id": "1c4a717bb672c42dce84fe4f3473f0d3ad80615e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2e3d0488adc81b49ef9aaebb0b3659035f1b3dec"
        },
        "date": 1662204587485,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 84089765,
            "range": "± 3053886",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 573778315,
            "range": "± 23207136",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 52157136,
            "range": "± 1786595",
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
          "id": "e4740f9f1460c8483f492e62cee78990267350d3",
          "message": "Allow alternative way to compute large scale diffs (#564)\n\n* Allow alternative way to compute large scale diffs\r\n\r\n* Update\r\n\r\n* Build with ubuntu-latest\r\n\r\n* Fix commands\r\n\r\n* Fix staging",
          "timestamp": "2022-09-03T13:11:01+01:00",
          "tree_id": "834983a2b5285918e4d89b4a2179cff80295c83e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/e4740f9f1460c8483f492e62cee78990267350d3"
        },
        "date": 1662207376274,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 88486108,
            "range": "± 4941577",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 551623869,
            "range": "± 17770697",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 49374604,
            "range": "± 1870525",
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
          "id": "8e44f3eadf947fbf6cd4ccba66a2d0aad739c246",
          "message": "Don't expand call with nested comment (#549)\n\n* Add test case\r\n\r\n* Don't expand function call with multiline comment\r\n\r\n* Changelog\r\n\r\n* Snapshots",
          "timestamp": "2022-09-04T17:46:37+01:00",
          "tree_id": "0c3f14f0378cfbc733eda661d1c43782d463f417",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8e44f3eadf947fbf6cd4ccba66a2d0aad739c246"
        },
        "date": 1662310285999,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71320580,
            "range": "± 2662878",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 482537774,
            "range": "± 7610485",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 45302903,
            "range": "± 1291421",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "75457809a88809ddc290a1c72179fe4e3610bb36",
          "message": "Update external test cases (#521)\n\n* Update external test cases\r\n\r\n* Update snapshots\r\n\r\nCo-authored-by: JohnnyMorganz <JohnnyMorganz@users.noreply.github.com>\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2022-09-04T18:16:03+01:00",
          "tree_id": "9bada0610e1937c31737e883c19fe6566c5e25dc",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/75457809a88809ddc290a1c72179fe4e3610bb36"
        },
        "date": 1662312031966,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67538409,
            "range": "± 851021",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 434767807,
            "range": "± 1066909",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 40977720,
            "range": "± 197946",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "filip.tibell@gmail.com",
            "name": "Filip Tibell",
            "username": "filiptibell"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f18a4aa18e2f593a25fee002fd6faf6db78b5745",
          "message": "Add search parent dirs config for VSCode extension (#568)",
          "timestamp": "2022-09-05T14:50:17+01:00",
          "tree_id": "dee4015ac08d86eb7a5f7e6c46a04dcdc03d105c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f18a4aa18e2f593a25fee002fd6faf6db78b5745"
        },
        "date": 1662386086261,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67293559,
            "range": "± 1057946",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 436671047,
            "range": "± 2382643",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 41045238,
            "range": "± 244394",
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
          "id": "e7ca1c2b0419bd290e1bea880565be2a49994602",
          "message": "Fix release build target (#569)",
          "timestamp": "2022-09-05T22:08:32+01:00",
          "tree_id": "8e9446e2a0ae963a2f81b568115e535b29603e5d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/e7ca1c2b0419bd290e1bea880565be2a49994602"
        },
        "date": 1662412370552,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 64890390,
            "range": "± 1017334",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 434563062,
            "range": "± 3083153",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 40706447,
            "range": "± 132031",
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
          "id": "3eaf1b603048039724547b6097e88acad0e0d647",
          "message": "Fix release path",
          "timestamp": "2022-09-05T22:16:51+01:00",
          "tree_id": "ab0697df9451310b4e8dfb9778b0c8b14604f9ef",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3eaf1b603048039724547b6097e88acad0e0d647"
        },
        "date": 1662412910605,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 78940500,
            "range": "± 5563525",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 549897156,
            "range": "± 42583965",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 50291418,
            "range": "± 2693582",
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
          "id": "e8fc6911cec657757b1ba5851cdb4c80553dacbf",
          "message": "Improve comments within function calls (#566)\n\n* Update test case\r\n\r\n* Expand call if it contains proper multiline comment\r\n\r\n* Update test case\r\n\r\n* Handle trailing comments on start parens\r\n\r\n* Add space after comment in start_parens\r\n\r\n* Handle leading comments on punctuation in Punctuated\r\n\r\n* Update snapshot",
          "timestamp": "2022-09-11T16:05:10+01:00",
          "tree_id": "cff279d528e087a2932b7e2de129fcbb4769f835",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/e8fc6911cec657757b1ba5851cdb4c80553dacbf"
        },
        "date": 1662908990777,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 72648249,
            "range": "± 2745476",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 496285813,
            "range": "± 9805492",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 46088676,
            "range": "± 1093941",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ian@iamthefij.com",
            "name": "Ian Fijolek",
            "username": "IamTheFij"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "133393cbcb2949cc62feff6738d4d72aaa7c11d7",
          "message": "Rename release targets and add linux-aarch64 (#559)\n\n* Add target for linux-aarch64\r\n\r\nI'm unsure if this will work in the build workflow.\r\n\r\n* Update asset names and update npm and vscode to use new names\r\n\r\n* Fix win name for npm-bin\r\n\r\n* Fix condition for artifact alias\r\n\r\n* Update pyproject to use new formatting\r\n\r\n* Make utils.ts prettier",
          "timestamp": "2022-09-12T14:33:50+01:00",
          "tree_id": "4484d16c94974fc5cfea97579183288ab37a0d2f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/133393cbcb2949cc62feff6738d4d72aaa7c11d7"
        },
        "date": 1662989934327,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 80626221,
            "range": "± 910686",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 520330565,
            "range": "± 11124329",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48620707,
            "range": "± 1087132",
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
          "id": "31da99148103cdfb9b51b17066c8502806e7d9d1",
          "message": "Fix workflow expression",
          "timestamp": "2022-09-12T14:51:02+01:00",
          "tree_id": "fc6373dd20f92932c14dc778b3093483f0442140",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/31da99148103cdfb9b51b17066c8502806e7d9d1"
        },
        "date": 1662990913929,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 64365382,
            "range": "± 496790",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 429804282,
            "range": "± 531446",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39573009,
            "range": "± 107998",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ian@iamthefij.com",
            "name": "Ian Fijolek",
            "username": "IamTheFij"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "91d473745feea4693ec31f02f3e5f3b2d92975c8",
          "message": "Fix aarch64 linux builds (#572)\n\nAdd c toolchain and instruct cargo to use the right linker for aarch64-linux",
          "timestamp": "2022-09-12T19:18:21+01:00",
          "tree_id": "77f213c0c434fed79013cc3920eae96b843da119",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/91d473745feea4693ec31f02f3e5f3b2d92975c8"
        },
        "date": 1663006963717,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 63898425,
            "range": "± 4991875",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 504090930,
            "range": "± 34337009",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 44348647,
            "range": "± 2974525",
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
          "id": "10922b9ac79c552fce7994b9ea81f78aeee9ba08",
          "message": "Fix workflow syntax",
          "timestamp": "2022-09-12T19:21:44+01:00",
          "tree_id": "fa5acbd9c29768336eb425c3af4514e38d847471",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/10922b9ac79c552fce7994b9ea81f78aeee9ba08"
        },
        "date": 1663007204240,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 82090128,
            "range": "± 2929695",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 509369498,
            "range": "± 4720486",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48533054,
            "range": "± 359841",
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
          "id": "09af8cbdd8d1073c35dd9b1323d6576b52807083",
          "message": "Update workflow for linker install",
          "timestamp": "2022-09-12T19:27:04+01:00",
          "tree_id": "79c7efc2c8a9beeb739de5d2da10d77d6312c0b7",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/09af8cbdd8d1073c35dd9b1323d6576b52807083"
        },
        "date": 1663007523528,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 80391051,
            "range": "± 892568",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 508054051,
            "range": "± 2469599",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 49298418,
            "range": "± 554502",
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
          "id": "5840654037db099324a5c0ff7b203f9b1e1b92e1",
          "message": "Add `--output-format=summary` (#575)\n\nAdd support for summary output",
          "timestamp": "2022-09-13T17:01:51+01:00",
          "tree_id": "4477bed3d85befda24423a68e0a81df9a8e7ea02",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5840654037db099324a5c0ff7b203f9b1e1b92e1"
        },
        "date": 1663085168778,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67234254,
            "range": "± 588034",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 430800375,
            "range": "± 651561",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39856477,
            "range": "± 275056",
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
          "id": "2d4bd31787a8e32d285f61ace45a6e77c1983b7b",
          "message": "Fix mistransformation of generic for comments (#580)\n\n* Fix incorrect token used in multiline generic for\r\n\r\n* Add test case\r\n\r\n* Fix check for comments in generic_for expr\r\n\r\n* Fix trivia location for multiline generic for\r\n\r\n* Add snapshot\r\n\r\n* Update changelog",
          "timestamp": "2022-09-19T19:57:00+01:00",
          "tree_id": "c64e8c97c4d447178f7691744d47839bcbba2ce0",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2d4bd31787a8e32d285f61ace45a6e77c1983b7b"
        },
        "date": 1663614146200,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 87517743,
            "range": "± 5930372",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 562116432,
            "range": "± 18013883",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 49938850,
            "range": "± 1463870",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "daad2db72e42deaffbee29f63b3c06c7a2f05fac",
          "message": "Update external test cases (#578)\n\n* Update external test cases\r\n\r\n* Update snapshots\r\n\r\nCo-authored-by: JohnnyMorganz <JohnnyMorganz@users.noreply.github.com>\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2022-09-19T20:13:43+01:00",
          "tree_id": "d6bee8b8e5ee014deb450b4a72a9c9c816e7ff46",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/daad2db72e42deaffbee29f63b3c06c7a2f05fac"
        },
        "date": 1663615093851,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67677563,
            "range": "± 944158",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 436822703,
            "range": "± 1515497",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 40865601,
            "range": "± 162615",
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
          "id": "dafa8f454f33cab5653f9243a4946b8ff1a2a750",
          "message": "Consider multiline comment when inlining function call (#581)\n\n* Add test case\r\n\r\n* Consider punctuation trivia in function args formatting\r\n\r\n* Add snapshot\r\n\r\n* Delete dead code",
          "timestamp": "2022-09-19T20:17:23+01:00",
          "tree_id": "8eb117b18e48453c13f06ff1ebd544ed12d0a6a1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/dafa8f454f33cab5653f9243a4946b8ff1a2a750"
        },
        "date": 1663615320189,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 70220790,
            "range": "± 1774012",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 440022434,
            "range": "± 1100618",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39812845,
            "range": "± 179676",
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
          "id": "c4c9f363ab40575e787fe548dcdae1803ca2a5b2",
          "message": "Take into account token width for if expression formatting (#583)\n\n* Add test case\r\n\r\n* Take into account token width for if expression\r\n\r\n* Update changelog\r\n\r\n* Ignore trivia in width calculation\r\n\r\n* Add snapshot",
          "timestamp": "2022-09-19T20:17:33+01:00",
          "tree_id": "dace5e27ffe5fd4182d86d21e72b545b54a244d4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c4c9f363ab40575e787fe548dcdae1803ca2a5b2"
        },
        "date": 1663615343451,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71128223,
            "range": "± 4880070",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 499474189,
            "range": "± 28724577",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 44907721,
            "range": "± 1986740",
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
          "id": "120ef08ceddbdeea94973a9971775aa81d0cfa99",
          "message": "Add Lua 5.3 and 5.4 support with full-moon update (#576)\n\n* Add feature flags\r\n\r\n* Update README\r\n\r\n* Update flags\r\n\r\n* Add test cases\r\n\r\n* Fix test name\r\n\r\n* Handle Lua 5.3 introductions\r\n\r\n* Update Lua 5.3 tests\r\n\r\n* Handle 5.4 attributes\r\n\r\n* Update snapshot\r\n\r\n* Update changelog\r\n\r\n* Update readme\r\n\r\n* Handle DoubleGreaterThan change\r\n\r\n* Allow unused mut\r\n\r\n* Run tests in CI\r\n\r\n* Temp full moon version\r\n\r\n* Use published full-moon",
          "timestamp": "2022-09-21T18:37:44+01:00",
          "tree_id": "a81588f51d016ceb9410dd2232f6503cf6c173f2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/120ef08ceddbdeea94973a9971775aa81d0cfa99"
        },
        "date": 1663782150833,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66841276,
            "range": "± 805668",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 418590236,
            "range": "± 2075906",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 41239637,
            "range": "± 437382",
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
          "id": "244da6ae64f4ee598d9af5bef8232a81a0a22324",
          "message": "extension: v1.4.0",
          "timestamp": "2022-09-21T18:40:54+01:00",
          "tree_id": "30b6759d6de9c67ae128204b8813a2b007bc8281",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/244da6ae64f4ee598d9af5bef8232a81a0a22324"
        },
        "date": 1663782414971,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 61602739,
            "range": "± 338572",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 410398903,
            "range": "± 2885720",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 38252988,
            "range": "± 150844",
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
          "id": "9e83eb85d0504d47260ba195b5c1e4ea1e963f25",
          "message": "v0.15.0",
          "timestamp": "2022-09-21T18:44:44+01:00",
          "tree_id": "32898273d5502e2fd9b2781d8345c67ada474b5b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/9e83eb85d0504d47260ba195b5c1e4ea1e963f25"
        },
        "date": 1663782648754,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 80534199,
            "range": "± 5489011",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 569530583,
            "range": "± 26671596",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 51630427,
            "range": "± 2637389",
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
          "id": "cf397fe963b10c00232cdcf404041e77d34f24bb",
          "message": "Update full-moon to fix parsing issues (#586)\n\n* Update full-moon\r\n\r\n* Temp full-moon\r\n\r\n* Proper full moon",
          "timestamp": "2022-09-22T16:38:36+01:00",
          "tree_id": "35f37ef8a4af5947649db0026219ce30bcf15d94",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/cf397fe963b10c00232cdcf404041e77d34f24bb"
        },
        "date": 1663861397418,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58491484,
            "range": "± 2506636",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 410797439,
            "range": "± 349370",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37320737,
            "range": "± 229569",
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
          "id": "d90c14dcdc6e4d7c438a32070101afb0a140ea8a",
          "message": "Parallelise test workflow jobs",
          "timestamp": "2022-09-22T16:38:51+01:00",
          "tree_id": "0972aef50fd804d7542b31661fddbf0866babce6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d90c14dcdc6e4d7c438a32070101afb0a140ea8a"
        },
        "date": 1663861468391,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 79416549,
            "range": "± 3786463",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 553695606,
            "range": "± 21444519",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48223219,
            "range": "± 1855674",
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
          "id": "c1e9c1dd0a8c585498764d44c7ce61556e2345ae",
          "message": "v0.15.1",
          "timestamp": "2022-09-22T16:41:27+01:00",
          "tree_id": "966cb9b2dc88faf392985a838b4e7c3b83bc919d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c1e9c1dd0a8c585498764d44c7ce61556e2345ae"
        },
        "date": 1663861761461,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57513537,
            "range": "± 598835",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 406054307,
            "range": "± 674711",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37799634,
            "range": "± 240973",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7bf884cc16efdaa4db75d7ceb2888bc161adbf55",
          "message": "Update external test cases (#589)\n\n* Update external test cases\r\n\r\n* Update snapshots\r\n\r\nCo-authored-by: JohnnyMorganz <JohnnyMorganz@users.noreply.github.com>\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2022-09-23T15:08:41+01:00",
          "tree_id": "a120d9808f0bca99dcadc50eb41199a829ba0f51",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7bf884cc16efdaa4db75d7ceb2888bc161adbf55"
        },
        "date": 1663942463878,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 80430483,
            "range": "± 4126227",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 544081127,
            "range": "± 20277591",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 47450237,
            "range": "± 1840393",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "82267684+uga-rosa@users.noreply.github.com",
            "name": "uga-rosa",
            "username": "uga-rosa"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4ad48d0a95b0324a64f6f58194dc26f7e84894ca",
          "message": "Update readme (collapse_simple_statement) (#590)",
          "timestamp": "2022-09-23T17:38:29+01:00",
          "tree_id": "fcc65a11e57ea1b91f54e05bf9cd43f47399e4f0",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4ad48d0a95b0324a64f6f58194dc26f7e84894ca"
        },
        "date": 1663951453327,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 75293361,
            "range": "± 4516281",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 526805554,
            "range": "± 9486672",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 47586817,
            "range": "± 2183134",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "silas.groh@t-online.de",
            "name": "Silas Groh",
            "username": "RubixDev"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ffec45427b20c73711006f74b66d3ec7816b91cc",
          "message": "Add `serialize`, `fromstr` and `wasm-bindgen` features (#592)\n\n* Add `serialize` feature\r\n\r\n* Add `fromstr` feature\r\n\r\n* Add `wasm-bindgen` feature\r\n\r\n* Don't enable `serialize` and `fromstr` features in release builds\r\n\r\n* Fix typo: lua53 -> lua52",
          "timestamp": "2022-10-09T14:37:41+01:00",
          "tree_id": "60d413f94b200b0464e462f95d1cbabd86f79f75",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ffec45427b20c73711006f74b66d3ec7816b91cc"
        },
        "date": 1665322997411,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 70729518,
            "range": "± 6035238",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 492819938,
            "range": "± 16479354",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 44737890,
            "range": "± 1618471",
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
          "id": "e4d6fd6b69cd0fd86400955cd6e6beea9289a1b7",
          "message": "Update insta snapshots when pulling test cases (#594)\n\n* Update insta snapshots when pulling test cases\r\n\r\n* Run on all features separately",
          "timestamp": "2022-10-09T14:58:32+01:00",
          "tree_id": "685401eb0a7baa84c368a203ee26544caa335c0e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/e4d6fd6b69cd0fd86400955cd6e6beea9289a1b7"
        },
        "date": 1665324208434,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59558756,
            "range": "± 500654",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 411421859,
            "range": "± 6987484",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37263685,
            "range": "± 83140",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kawarimidoll+git@gmail.com",
            "name": "カワリミ人形",
            "username": "kawarimidoll"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "62db6d84947d3710ef11e7fc8ce5f1a7d2ec7d65",
          "message": "Add default collapse_simple_statement to README.md (#598)\n\nUpdate README.md",
          "timestamp": "2022-10-10T11:50:51+01:00",
          "tree_id": "9961c51b0c07818d1844dc245bd33dc42ffa73e7",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/62db6d84947d3710ef11e7fc8ce5f1a7d2ec7d65"
        },
        "date": 1665399341342,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58751818,
            "range": "± 968956",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 410575780,
            "range": "± 472315",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37323861,
            "range": "± 1625937",
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
          "id": "47138aa82652a82b68c5b29037c740bbbe85aac4",
          "message": "Cleanup all snapshots (#601)\n\n* Cleanup all snapshots\r\n\r\n* More snapshots cleanup",
          "timestamp": "2022-10-10T13:25:09+01:00",
          "tree_id": "5a7aaf981b8cd173834d3a29044ba88e315a5f25",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/47138aa82652a82b68c5b29037c740bbbe85aac4"
        },
        "date": 1665405044410,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 73893884,
            "range": "± 2215273",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 483679176,
            "range": "± 8109245",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48523466,
            "range": "± 446816",
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
          "id": "b8c32f7ca79aa56f3585455fd664ea24cee19ff0",
          "message": "Add hang level to hanging if expression (#599)\n\n* Add test case\r\n\r\n* Add a hang level for hanging if expression\r\n\r\n* Update snapshot\r\n\r\n* Add test case\r\n\r\n* Calculate hang level properly\r\n\r\n* Update snapshot\r\n\r\n* Update changelog\r\n\r\n* Add negated assigns test",
          "timestamp": "2022-10-15T15:15:28+01:00",
          "tree_id": "054ebcda954f61326b9f609308fbaa1e2f170343",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b8c32f7ca79aa56f3585455fd664ea24cee19ff0"
        },
        "date": 1665843691553,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 75672658,
            "range": "± 4720604",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 516601882,
            "range": "± 22372061",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 46542351,
            "range": "± 2262830",
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
          "id": "bee202b6fdaffdf4a19a84f75746b796a063a764",
          "message": "Don't collapse when comment present in typeinfo tuple (#612)\n\n* Add test case\r\n\r\n* Check for comments in type info tuple\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2022-10-27T12:55:17+01:00",
          "tree_id": "e59736a40d75fad3bf63556ce5a35f0de8e36d68",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bee202b6fdaffdf4a19a84f75746b796a063a764"
        },
        "date": 1666872085206,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71053838,
            "range": "± 1695900",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 464721088,
            "range": "± 9058656",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 45226950,
            "range": "± 1252190",
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
          "id": "d681afb314e52eb4850211ebf06acc3cd709b681",
          "message": "Don't remove excess parentheses which are highlighting precedence (#610)\n\n* Add test case\r\n\r\n* Don't remove parentheses when highlighting precedence\r\n\r\n* Update snapshots\r\n\r\n* Update changelog\r\n\r\n* Update snapshots",
          "timestamp": "2022-10-27T13:10:07+01:00",
          "tree_id": "7a3c0dfc264761caca846e1d66e1974aa2048f95",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d681afb314e52eb4850211ebf06acc3cd709b681"
        },
        "date": 1666872912735,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59504544,
            "range": "± 507525",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 410359000,
            "range": "± 2273403",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37229401,
            "range": "± 178629",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}