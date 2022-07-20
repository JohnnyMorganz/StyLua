window.BENCHMARK_DATA = {
  "lastUpdate": 1658346438621,
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
      }
    ]
  }
}