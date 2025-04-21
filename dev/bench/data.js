window.BENCHMARK_DATA = {
  "lastUpdate": 1745249587582,
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
      },
      {
        "commit": {
          "author": {
            "email": "k.trzesniewski@gmail.com",
            "name": "Chris Trześniewski",
            "username": "ktrz"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c3136e0a60568da1ce86a136b02b93e7b80535bd",
          "message": "fix: 614 support all features in wasm (#615)\n\n* fix: 614 support all features in wasm\r\n\r\n* pass all --features to cargo build in build-wasm.sh\r\n\r\n* update changelog\r\n\r\n* Update CHANGELOG.md\r\n\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2022-10-28T17:50:10+01:00",
          "tree_id": "0e6d031e78bbdec8564d271a166855f3ad56f398",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c3136e0a60568da1ce86a136b02b93e7b80535bd"
        },
        "date": 1666976173291,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71192621,
            "range": "± 3683823",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 511578202,
            "range": "± 14727649",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 45845043,
            "range": "± 2378686",
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
          "id": "a54d6bc3955f42eadd70bbc85ece8417c017cf9e",
          "message": "v0.15.2",
          "timestamp": "2022-10-31T09:55:48Z",
          "tree_id": "c1fc102fee13947986574a9550e122d42c1e26fc",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a54d6bc3955f42eadd70bbc85ece8417c017cf9e"
        },
        "date": 1667210533312,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71321381,
            "range": "± 1202481",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 476494889,
            "range": "± 2466335",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 45793828,
            "range": "± 550056",
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
          "id": "56d1f9a1bcc801274cac8a5597e02fcd15ee936a",
          "message": "Fix necessary parentheses removed in `(-X) ^ Y` (#624)\n\n* Add test case\r\n\r\n* Handle exponent precedence\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2022-11-29T19:20:41Z",
          "tree_id": "96f9380e4de9f6129c43a6dd50115972b2b5f11d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/56d1f9a1bcc801274cac8a5597e02fcd15ee936a"
        },
        "date": 1669749943606,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 61525087,
            "range": "± 579134",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 412701666,
            "range": "± 794279",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 36329202,
            "range": "± 162677",
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
          "id": "399aab74503cef34a8015f95ff4a88eca0e2e6ea",
          "message": "Take into account `function` token in anon func (#625)",
          "timestamp": "2022-11-29T20:24:32Z",
          "tree_id": "5bf0df8f8d3679ccd77c717e58be590270fa960d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/399aab74503cef34a8015f95ff4a88eca0e2e6ea"
        },
        "date": 1669753832348,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 73924439,
            "range": "± 3864723",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 519683406,
            "range": "± 28107964",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 43435325,
            "range": "± 2255334",
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
          "id": "b91829c7986fe363a288a2ec5a64493c61ee1702",
          "message": "Luau: Handle comments inside of array types (#626)\n\n* Add test case\r\n\r\n* Handle comments inside of array types\r\n\r\n* Update changelog\r\n\r\n* Update snapshot",
          "timestamp": "2022-11-29T21:40:38Z",
          "tree_id": "c72c834a3f878599358e61d382f945d2cc1e9b9b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b91829c7986fe363a288a2ec5a64493c61ee1702"
        },
        "date": 1669758406420,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 75350684,
            "range": "± 2742104",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 527444083,
            "range": "± 17818168",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 45148361,
            "range": "± 1317743",
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
          "id": "ffbef7ea262063d3eeb5c67232bfdfa2e978b87a",
          "message": "Fix clippy warnings",
          "timestamp": "2022-11-29T21:49:13Z",
          "tree_id": "869d592026f1f0e4955428ac258076056c317e7c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ffbef7ea262063d3eeb5c67232bfdfa2e978b87a"
        },
        "date": 1669758861933,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65469621,
            "range": "± 598542",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 417123112,
            "range": "± 1642469",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 38621528,
            "range": "± 354680",
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
          "id": "4d47214271251adf4dce58a793d588c422b388db",
          "message": "v0.15.3",
          "timestamp": "2022-12-07T13:13:22Z",
          "tree_id": "815294c2fef09687c88dadac242f96c4dd886ad0",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4d47214271251adf4dce58a793d588c422b388db"
        },
        "date": 1670419153749,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66712547,
            "range": "± 535960",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 419319682,
            "range": "± 4149296",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39062062,
            "range": "± 361846",
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
          "id": "9045b9f8ed1b957990dbc0671a627969586f1f0d",
          "message": "Remove unnecessary parentheses around Luau types (#632)\n\n* Add test case\r\n\r\n* Remove unnecessary parens\r\n\r\n* Update snapshots",
          "timestamp": "2023-01-04T18:19:23Z",
          "tree_id": "0db168b934b0c98c8c7120c2eae60c11d1e2a0c9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/9045b9f8ed1b957990dbc0671a627969586f1f0d"
        },
        "date": 1672856612414,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59617479,
            "range": "± 457097",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 415458353,
            "range": "± 2509815",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 36893788,
            "range": "± 121285",
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
          "id": "3699358f0ceebe6651789cff25f289d3ac84c937",
          "message": "Update changelog for #632",
          "timestamp": "2023-01-04T20:06:02Z",
          "tree_id": "11ccec429dc2ac1d9ace1cf96e1c459e4e6e2f6e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3699358f0ceebe6651789cff25f289d3ac84c937"
        },
        "date": 1672863026963,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59041328,
            "range": "± 186292",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 415752660,
            "range": "± 3389075",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37054991,
            "range": "± 478952",
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
          "id": "1c6e1635955900086a0dc80cff25e6bb40db47bb",
          "message": "Update full-moon to 0.17 (#634)\n\nUpdate full moon",
          "timestamp": "2023-01-04T20:16:44Z",
          "tree_id": "ac8092b2cd76e7caea2783a14d1f5011843e169f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1c6e1635955900086a0dc80cff25e6bb40db47bb"
        },
        "date": 1672863659635,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 64596481,
            "range": "± 412758",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 419956576,
            "range": "± 742926",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 38161599,
            "range": "± 315664",
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
          "id": "eca166397c9cfccb7212de9659721d1bd4579666",
          "message": "Collapse `goto` as a simple statement (#631)\n\n* Add test case\r\n\r\n* Collapse a goto as simple statement\r\n\r\n* Update test case\r\n\r\n* Update changelog",
          "timestamp": "2023-01-04T20:16:32Z",
          "tree_id": "4b5093b5d17f7ea549128bffdabd3077510540a4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/eca166397c9cfccb7212de9659721d1bd4579666"
        },
        "date": 1672863730737,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 92434203,
            "range": "± 3105244",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 625216942,
            "range": "± 15076458",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 54413864,
            "range": "± 1735150",
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
          "id": "ff25837ad58cc0ae6ec0930984e98dadc1080845",
          "message": "Support interpolated string nodes (#635)\n\n* Handle interpolated string support\r\n\r\n* fix\r\n\r\n* undo\r\n\r\n* fix",
          "timestamp": "2023-01-04T21:12:24Z",
          "tree_id": "eac1e0b2f930fd6d005f4c8902b906814d700249",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ff25837ad58cc0ae6ec0930984e98dadc1080845"
        },
        "date": 1672866986589,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 62981651,
            "range": "± 1204658",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 417183093,
            "range": "± 3272422",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 38025385,
            "range": "± 342007",
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
          "id": "3e3fd03107628f5dea27f6dea283faa3592303c0",
          "message": "Update external test cases (#593)\n\nCo-authored-by: JohnnyMorganz <JohnnyMorganz@users.noreply.github.com>",
          "timestamp": "2023-01-04T21:26:52Z",
          "tree_id": "27f18b2016b5498107365132cce575aeef8234b0",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3e3fd03107628f5dea27f6dea283faa3592303c0"
        },
        "date": 1672867914703,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 80560596,
            "range": "± 2255887",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 554406557,
            "range": "± 18403180",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 48227599,
            "range": "± 2762907",
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
          "id": "94732ecbc38cc9cb248643ac83ba9022c24ac81e",
          "message": "Profiling improvements (#591)\n\n* Remove call to contains_comment in return\r\n\r\n* Defer should_expand in table constructor to end\r\n\r\n* Table fixes\r\n\r\n* Use cheaper expression inline comment detection\r\n\r\n* Panic\r\n\r\n* Remove unnecessary clone\r\n\r\n* Improve assignment shape calculation\r\n\r\n* Update changelog\r\n\r\n* Revert \"Improve assignment shape calculation\"\r\n\r\nThis reverts commit 01c72a67655e3cbae934cb4e4abf57904881459c.",
          "timestamp": "2023-01-12T13:29:49Z",
          "tree_id": "ac132b8c77bf7942b2a2b4d3e27dd0fa7222a606",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/94732ecbc38cc9cb248643ac83ba9022c24ac81e"
        },
        "date": 1673530461633,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56878945,
            "range": "± 308242",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370169993,
            "range": "± 2358723",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28133534,
            "range": "± 209414",
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
          "id": "9564b3f8638c9516cec6d73a37b68747c7112d1c",
          "message": "Add snapshot test for #627\n\nFixes #627",
          "timestamp": "2023-01-12T13:32:08Z",
          "tree_id": "cdaf48c9b80e472ed149413128f75c87a1a39577",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/9564b3f8638c9516cec6d73a37b68747c7112d1c"
        },
        "date": 1673530597879,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54128152,
            "range": "± 737478",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369532304,
            "range": "± 1210311",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27329409,
            "range": "± 47757",
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
          "id": "540ecfb832e3bccf86e0e037b9c855aa464fc4e7",
          "message": "Fixed malformed formatting when newline present after return token (#639)\n\n* Add test case\r\n\r\n* Update test case\r\n\r\n* Fix comment check\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2023-01-12T13:47:32Z",
          "tree_id": "dfb1d3fedcc521945bd8ea6032450a83edca4c80",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/540ecfb832e3bccf86e0e037b9c855aa464fc4e7"
        },
        "date": 1673531516414,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55467285,
            "range": "± 241924",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371884846,
            "range": "± 1819090",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28046847,
            "range": "± 85063",
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
          "id": "8da1a5ff721b19ab875356922c2d000cc7bcf930",
          "message": "Fixed malformed formatting of punctuated expressions list with comments (#640)\n\n* Add test case\r\n\r\n* Update prepend_newline_indent\r\n\r\n* Use prepend_newline_indent when formatting punctuated multiline\r\n\r\n* Fix puncutated contains inline comments check\r\n\r\n* Fix checks in assignment and returns formatting\r\n\r\n* Update changelog\r\n\r\n* Update snapshot\r\n\r\n* Fix checks\r\n\r\n* Update test case\r\n\r\n* Fix\r\n\r\n* Add another test case",
          "timestamp": "2023-01-12T16:16:21Z",
          "tree_id": "9e5ece92533b368b7777bc2c355944866510a1f1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8da1a5ff721b19ab875356922c2d000cc7bcf930"
        },
        "date": 1673540450381,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54368548,
            "range": "± 662827",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370054491,
            "range": "± 1300424",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27468333,
            "range": "± 98904",
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
          "id": "10d77fea46f280d32e92cb0ba8cc67166d7f6ef9",
          "message": "Update CHANGELOG",
          "timestamp": "2023-01-12T16:18:13Z",
          "tree_id": "a73208813a27099416185ad1b9fd5da14d634aaf",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/10d77fea46f280d32e92cb0ba8cc67166d7f6ef9"
        },
        "date": 1673540567722,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54176609,
            "range": "± 416737",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369406069,
            "range": "± 684244",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27377498,
            "range": "± 145244",
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
          "id": "8a0bcd7f953b1cfda04bc3092046ade87d0ac90d",
          "message": "Add CorePackages to full scale testing (#641)",
          "timestamp": "2023-01-12T22:08:30Z",
          "tree_id": "832fa8b4b0a6355e9e438dabb879c28cc8626039",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8a0bcd7f953b1cfda04bc3092046ade87d0ac90d"
        },
        "date": 1673561622126,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66823494,
            "range": "± 918834",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 429138684,
            "range": "± 2158022",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 33710670,
            "range": "± 259392",
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
          "id": "8c2763eaa8c8304e014327682d84e8376a6e45c6",
          "message": "Dont hang `local x = function` assignments (Take 2) (#600)\n\n* Prevent hanging anon functions\r\n\r\n* Fix shape for anon function formatting\r\n\r\n* Add test case\r\n\r\n* Update snapshots",
          "timestamp": "2023-01-14T18:22:18Z",
          "tree_id": "3f73b3a7d7afd53a285f334cec92babbc45b69ac",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8c2763eaa8c8304e014327682d84e8376a6e45c6"
        },
        "date": 1673720772805,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55154773,
            "range": "± 592366",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371220287,
            "range": "± 1438252",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27489823,
            "range": "± 106980",
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
          "id": "3afdd788be23588d42b8411b2cce3d1f73d9c3f2",
          "message": "Update changelog for #600",
          "timestamp": "2023-01-14T18:23:38Z",
          "tree_id": "6aab409980512a7527a9b5d4160b143c3c5bbbdf",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3afdd788be23588d42b8411b2cce3d1f73d9c3f2"
        },
        "date": 1673720860951,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54436535,
            "range": "± 564314",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371849715,
            "range": "± 1485661",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27443392,
            "range": "± 127211",
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
          "id": "1026fc20f8d5c4e91f8d74d368d21c96216307ff",
          "message": "v0.16.0",
          "timestamp": "2023-01-15T12:23:39Z",
          "tree_id": "eeef0ec2fd4560203b32d2e46542da3ff19de218",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1026fc20f8d5c4e91f8d74d368d21c96216307ff"
        },
        "date": 1673785674643,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53482099,
            "range": "± 406287",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369530455,
            "range": "± 1073761",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28108490,
            "range": "± 58047",
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
          "id": "76d74721066c5a0e8289dab40a834a1cee1921fc",
          "message": "Fix clippy lints (#650)\n\nFix clippy warnings",
          "timestamp": "2023-02-10T21:31:41Z",
          "tree_id": "bbb5fda8057a61c1eca7213fdc001a0c0cca9af2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/76d74721066c5a0e8289dab40a834a1cee1921fc"
        },
        "date": 1676064979240,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55737597,
            "range": "± 350618",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368533389,
            "range": "± 638822",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27737767,
            "range": "± 299282",
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
          "id": "ad14ef49c492cbf13c5e0c7c54f5bad43524dc56",
          "message": "Hang multiline function argument containing comments (#649)\n\n* Add test case\r\n\r\n* Hang a multiline function arg containing a comment\r\n\r\n* Update snapshot\r\n\r\n* Update changelog",
          "timestamp": "2023-02-10T21:40:32Z",
          "tree_id": "a226a2330dc796e8dd16f6c01de57ed81bd4cc98",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ad14ef49c492cbf13c5e0c7c54f5bad43524dc56"
        },
        "date": 1676065485010,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56738016,
            "range": "± 369228",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368182200,
            "range": "± 1325324",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28779707,
            "range": "± 317006",
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
          "id": "bf53cb2633adb898bc9140ca7a0fe0e35726ce2b",
          "message": "Massage Verify AST for removed type parentheses (#651)\n\n* Massage verify ast for removed type parentheses\r\n\r\n* Update changelog\r\n\r\n* Include link",
          "timestamp": "2023-02-10T21:48:12Z",
          "tree_id": "2ae44cd79cb13286f54130f6b9271916d7916b5b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bf53cb2633adb898bc9140ca7a0fe0e35726ce2b"
        },
        "date": 1676065949580,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57638576,
            "range": "± 301005",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 367851581,
            "range": "± 697396",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28879375,
            "range": "± 195128",
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
          "id": "12d3f3d31b203d90d615183d6ac356b3cb71b913",
          "message": "v0.16.1",
          "timestamp": "2023-02-10T21:55:06Z",
          "tree_id": "43c2c35e7a5d225aaffbd5fde57a3b3387fd6e50",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/12d3f3d31b203d90d615183d6ac356b3cb71b913"
        },
        "date": 1676066385219,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57038432,
            "range": "± 443691",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 367628346,
            "range": "± 1924537",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28604336,
            "range": "± 333678",
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
          "id": "06a761e197e61072af1bf2379be4a3f1f11a22b5",
          "message": "Sort requires (#653)\n\n* Setup entry point\r\n\r\n* Create tests\r\n\r\n* Sort requires system\r\n\r\n* Update test snapshots\r\n\r\n* Update readme\r\n\r\n* Handle trivia appropriately\r\n\r\n* More test cases\r\n\r\n* Add sort services system\r\n\r\n* Move out sort requires\r\n\r\n* Update changelog\r\n\r\n* Add variable case test\r\n\r\n* Respect `-- stylua: ignore` comments\r\n\r\n* Add more test cases",
          "timestamp": "2023-02-12T13:06:20Z",
          "tree_id": "2d9f5905b611095db96a962e31b8fbdeba285272",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/06a761e197e61072af1bf2379be4a3f1f11a22b5"
        },
        "date": 1676207449243,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56302372,
            "range": "± 966225",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368424178,
            "range": "± 593782",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28252985,
            "range": "± 336132",
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
          "id": "15c1f0d4880dbcfe37dd2828da10745f95a13825",
          "message": "Remove public visibility of sort requires module",
          "timestamp": "2023-02-13T11:30:24Z",
          "tree_id": "4e5743f9665820829ec27c3d32b99e597789d11d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/15c1f0d4880dbcfe37dd2828da10745f95a13825"
        },
        "date": 1676288125451,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58279894,
            "range": "± 2769002",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 414916394,
            "range": "± 18275855",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30835976,
            "range": "± 721613",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "github@lei.sh",
            "name": "Guillaume",
            "username": "LEI"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d6ed0519ca493a3a17c98367147fd77d230cd044",
          "message": "Add default `editorconfig` feature (#645)\n\nAdd default `editorconfig` feature (#645)",
          "timestamp": "2023-02-27T11:24:48Z",
          "tree_id": "8bf2e31f5f1e2d7cbd108fb58249c39a93d38f11",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d6ed0519ca493a3a17c98367147fd77d230cd044"
        },
        "date": 1677497358136,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 52619586,
            "range": "± 560566",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368600328,
            "range": "± 1114538",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27231339,
            "range": "± 47865",
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
          "id": "2225d42ca5d8e76d849d4a9b5b79c71cea8e11de",
          "message": "extension: Pass cwd as workspace folder for version check (#659)",
          "timestamp": "2023-03-09T14:05:41Z",
          "tree_id": "007efa3ce97c0c39e181368c971bd238950d47cc",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2225d42ca5d8e76d849d4a9b5b79c71cea8e11de"
        },
        "date": 1678371048340,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55133272,
            "range": "± 394492",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368731669,
            "range": "± 990193",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27159818,
            "range": "± 72819",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "erik.b22@gmail.com",
            "name": "Erik Berkun-Drevnig",
            "username": "eberkund"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b79dd0869e1652c9f9414c7e474acc9f56d7726f",
          "message": "Add Dockerfile (#655)\n\n* Add Dockerfile\r\n\r\n* Add cargo flags and GHA workflow\r\n\r\n* update README\r\n\r\n* Add login action\r\n\r\n* Update .github/workflows/docker.yml\r\n\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>\r\n\r\n* move docker job to release.yml\r\n\r\n* Update Docker workflow\r\n\r\n* always push\r\n\r\n---------\r\n\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2023-03-11T16:40:59Z",
          "tree_id": "0af204f14a0a081fb319dea0fc418902ab5b569f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b79dd0869e1652c9f9414c7e474acc9f56d7726f"
        },
        "date": 1678553240860,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 73644850,
            "range": "± 2785356",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 509031145,
            "range": "± 15488291",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 39586509,
            "range": "± 908276",
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
          "id": "4ebcd90d1729fb3da234f746bbf000af5420ab0e",
          "message": "Update CHANGELOG",
          "timestamp": "2023-03-11T16:44:28Z",
          "tree_id": "ff52d1dcced0d0d9bfa699a3764e7aed038865c5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4ebcd90d1729fb3da234f746bbf000af5420ab0e"
        },
        "date": 1678553429505,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 78189564,
            "range": "± 5012541",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 525764881,
            "range": "± 22164573",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 38315197,
            "range": "± 1487592",
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
          "id": "bd00aae4c9dea256209dbb124a82834904bd0c8a",
          "message": "v0.17.0",
          "timestamp": "2023-03-11T16:46:37Z",
          "tree_id": "6d6a361c18ba78d74ff572efedfe66bab4536293",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bd00aae4c9dea256209dbb124a82834904bd0c8a"
        },
        "date": 1678553511880,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55669763,
            "range": "± 556992",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370571422,
            "range": "± 441199",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27273158,
            "range": "± 98899",
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
          "id": "efa0ae5fc1eeed420c0f3197e7922e121c616146",
          "message": "extension: v1.5.0",
          "timestamp": "2023-03-11T16:48:41Z",
          "tree_id": "74da556b5a42298c04458a3f89bc6ede64fbd4e5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/efa0ae5fc1eeed420c0f3197e7922e121c616146"
        },
        "date": 1678554032144,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54703989,
            "range": "± 710988",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370066416,
            "range": "± 731381",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27336173,
            "range": "± 223039",
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
          "id": "7155c55631e479332b449c8671d30f5d9321c7fa",
          "message": "Upgrade full-moon to v0.18.0",
          "timestamp": "2023-03-14T19:23:32Z",
          "tree_id": "d5411deba2a1eb621686acf83d757407cf1740b8",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7155c55631e479332b449c8671d30f5d9321c7fa"
        },
        "date": 1678822205440,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 68283615,
            "range": "± 2556804",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 491950091,
            "range": "± 11911155",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37563028,
            "range": "± 970511",
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
          "id": "1fe8b21a49aaab596e0076fb338e0329d0adfaed",
          "message": "Upgrade full-moon to v0.18.1",
          "timestamp": "2023-03-19T20:01:35Z",
          "tree_id": "ac59e0a502707c184e69a20202e4f176c884afa5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1fe8b21a49aaab596e0076fb338e0329d0adfaed"
        },
        "date": 1679256457492,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 71090034,
            "range": "± 2238110",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 480190204,
            "range": "± 8155802",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37833376,
            "range": "± 1313677",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bf@benfrain.com",
            "name": "Ben Frain",
            "username": "benfrain"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fd8a6b10d5bb05dc41d5cb83f563bc6497ba2897",
          "message": "Adding Homebrew instructions (#661)\n\n* Adding Homebrew instructions\r\n\r\nFollowed https://github.com/JohnnyMorganz/StyLua/issues/237 some time ago and noticed the README never got updated. Instructions for homebrew now in.\r\n\r\n* Update README.md\r\n\r\nfixing typo",
          "timestamp": "2023-03-23T14:48:49Z",
          "tree_id": "a9fd5b904a13a97c3e91999c8c714a2d0cc4052a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fd8a6b10d5bb05dc41d5cb83f563bc6497ba2897"
        },
        "date": 1679583294328,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 69562578,
            "range": "± 2803067",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 479904538,
            "range": "± 12099262",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 35480342,
            "range": "± 899760",
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
          "id": "7c64fcad82917d788aad19ddd648e293a8042ddc",
          "message": "Accept all snapshots when pulling latest test cases",
          "timestamp": "2023-03-30T13:25:18+01:00",
          "tree_id": "db0b4b80ad819b969236f61bf138b59f510560c5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7c64fcad82917d788aad19ddd648e293a8042ddc"
        },
        "date": 1680179440059,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56978844,
            "range": "± 327798",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 372316823,
            "range": "± 1088765",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28643605,
            "range": "± 138074",
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
          "id": "46457ad4e4130d07ee0f9a5cf95ac10023c8ceeb",
          "message": "Fix comments in punctuated list for returns and assignments (#663)\n\n* Add test cases\r\n\r\n* Properly handle comments when formatting a punctuated list multiline\r\n\r\n* Update snapshots\r\n\r\n* Hang equals token for comments\r\n\r\n* Update changelog",
          "timestamp": "2023-03-30T13:48:55+01:00",
          "tree_id": "b1f1300a4347819be85c1e17c042ce551df24308",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/46457ad4e4130d07ee0f9a5cf95ac10023c8ceeb"
        },
        "date": 1680180879424,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58059556,
            "range": "± 1749068",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 383089516,
            "range": "± 12426483",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30244710,
            "range": "± 993618",
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
          "id": "c3715721edf6108bcfd0ef0a12a7e68647b6ae78",
          "message": "Cleanup trivia code (#664)\n\n* Cleanup trivia code\r\n\r\n* Fix imports\r\n\r\n* Fixes\r\n\r\n* Cleanup\r\n\r\n* Cleanup",
          "timestamp": "2023-03-30T16:46:50+01:00",
          "tree_id": "e686652695667159fc3ee5460d8647199bcfcdf9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c3715721edf6108bcfd0ef0a12a7e68647b6ae78"
        },
        "date": 1680191558098,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57142931,
            "range": "± 2405000",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 401327665,
            "range": "± 15196231",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30852453,
            "range": "± 1074000",
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
          "id": "b8c65f3a49e19a672b21f7248726d843e46a5296",
          "message": "Format line endings in multiline strings and comments (#666)\n\n* Add test case\r\n\r\n* Convert line endings\r\n\r\n* Update changelog",
          "timestamp": "2023-03-30T16:47:23+01:00",
          "tree_id": "d5374827fecaa4e95e22ea65ca0ebe1ab29819fe",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b8c65f3a49e19a672b21f7248726d843e46a5296"
        },
        "date": 1680191560043,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53799146,
            "range": "± 516084",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370181641,
            "range": "± 3123580",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28034449,
            "range": "± 164811",
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
          "id": "6cb912b2f4f2a5f9cbf5e8f4b11779e9aa8d7bf6",
          "message": "v0.17.1",
          "timestamp": "2023-03-30T16:49:39+01:00",
          "tree_id": "fea4372474b217e6f4afa41538ee67a2f6d03093",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6cb912b2f4f2a5f9cbf5e8f4b11779e9aa8d7bf6"
        },
        "date": 1680191777289,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 70186448,
            "range": "± 5646180",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 474005123,
            "range": "± 17284621",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 33665599,
            "range": "± 1777447",
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
          "id": "aefde486d75d4d355521a4e0e551581b73ade876",
          "message": "Add option `--sort-requires`\n\nFixes #669",
          "timestamp": "2023-04-10T18:41:11+01:00",
          "tree_id": "c60911a24548438039b15c96a702e58b12a6d411",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/aefde486d75d4d355521a4e0e551581b73ade876"
        },
        "date": 1681148759976,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53966025,
            "range": "± 475359",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370470551,
            "range": "± 1681091",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28672061,
            "range": "± 132980",
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
          "id": "6b98a135443d41124969fd13cd198c36a9c5ae82",
          "message": "Test wasm-pack building on CI",
          "timestamp": "2023-04-10T18:45:34+01:00",
          "tree_id": "6310df22536b37f0fb3b5e144ca0c75da800b968",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6b98a135443d41124969fd13cd198c36a9c5ae82"
        },
        "date": 1681149043009,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58225620,
            "range": "± 3288770",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 386369132,
            "range": "± 13698849",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30093048,
            "range": "± 856894",
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
          "id": "6454d562e1923c2388a08aee277ff0c834adf20f",
          "message": "Update CI versions",
          "timestamp": "2023-04-10T18:50:58+01:00",
          "tree_id": "c4b2b60837878529731174048281639a61b96d86",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6454d562e1923c2388a08aee277ff0c834adf20f"
        },
        "date": 1681149397793,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 62164989,
            "range": "± 3157869",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 444654123,
            "range": "± 8946754",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34623053,
            "range": "± 1525907",
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
          "id": "846d3c7af3aafe76d5c0b31078e9777fb817bf4c",
          "message": "Create .github/dependabot.yml",
          "timestamp": "2023-04-10T18:54:01+01:00",
          "tree_id": "aa4f33f1dfacec6aeec47c59a164c17e1edbe148",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/846d3c7af3aafe76d5c0b31078e9777fb817bf4c"
        },
        "date": 1681149612028,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55689661,
            "range": "± 392032",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 372541467,
            "range": "± 1663512",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28739597,
            "range": "± 270561",
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
          "id": "5c2577f238e162589287d63f55dc0ad4d5b585c2",
          "message": "Update actions-rs and wasm-pack build",
          "timestamp": "2023-04-10T18:59:51+01:00",
          "tree_id": "eba7d072bd43cc14ae3c2cec80c456190d6508dc",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5c2577f238e162589287d63f55dc0ad4d5b585c2"
        },
        "date": 1681149904235,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65305112,
            "range": "± 1265286",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 445501006,
            "range": "± 4985085",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 33844158,
            "range": "± 168642",
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
          "id": "05bacfd92e364c57efead3e386db66acfdf973f2",
          "message": "Downgrade wasm-pack version",
          "timestamp": "2023-04-10T19:00:47+01:00",
          "tree_id": "bb6c3b320b8e69b72213cda433bf01bc918bbe80",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/05bacfd92e364c57efead3e386db66acfdf973f2"
        },
        "date": 1681149930373,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55092871,
            "range": "± 571189",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371494874,
            "range": "± 2874058",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28102218,
            "range": "± 100070",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "beae50208e44751f68145480b6aaab230150f528",
          "message": "Bump regex from 1.5.4 to 1.7.3 (#678)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.5.4 to 1.7.3.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.5.4...1.7.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:06:29+01:00",
          "tree_id": "858860dec96b414398b725c1ac3f6250d170884a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/beae50208e44751f68145480b6aaab230150f528"
        },
        "date": 1681150262859,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53840283,
            "range": "± 793832",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369123749,
            "range": "± 1267163",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27969772,
            "range": "± 73609",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3e2ee5d98730b83c8cc4ceab57e4bc9cc3ad110f",
          "message": "Bump serde_json from 1.0.79 to 1.0.95 (#677)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.79 to 1.0.95.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.79...v1.0.95)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:06:42+01:00",
          "tree_id": "0301bad4d5d26b9faad6f744609aec27a748a6ac",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3e2ee5d98730b83c8cc4ceab57e4bc9cc3ad110f"
        },
        "date": 1681150288043,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55012252,
            "range": "± 849531",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368950623,
            "range": "± 1038192",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28844266,
            "range": "± 275405",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7cf70577a5be6efde4e9da215fc0cc720ede2fd1",
          "message": "Bump peter-evans/create-pull-request from 3 to 5 (#675)\n\nBumps [peter-evans/create-pull-request](https://github.com/peter-evans/create-pull-request) from 3 to 5.\r\n- [Release notes](https://github.com/peter-evans/create-pull-request/releases)\r\n- [Commits](https://github.com/peter-evans/create-pull-request/compare/v3...v5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: peter-evans/create-pull-request\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:07:36+01:00",
          "tree_id": "fd26ceb34615769a2ee43332d5e66c3b540d75fe",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7cf70577a5be6efde4e9da215fc0cc720ede2fd1"
        },
        "date": 1681150339551,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55783041,
            "range": "± 774924",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371467724,
            "range": "± 668078",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29225061,
            "range": "± 430137",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e9816e692d77b576422614448cc6d8940a1bd2d0",
          "message": "Bump bumpalo from 3.7.0 to 3.12.0 (#670)\n\nBumps [bumpalo](https://github.com/fitzgen/bumpalo) from 3.7.0 to 3.12.0.\r\n- [Release notes](https://github.com/fitzgen/bumpalo/releases)\r\n- [Changelog](https://github.com/fitzgen/bumpalo/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/fitzgen/bumpalo/compare/3.7.0...3.12.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: bumpalo\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:08:19+01:00",
          "tree_id": "6849ca270d2dea74c513ecb86db3c2ed6acfcf6a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/e9816e692d77b576422614448cc6d8940a1bd2d0"
        },
        "date": 1681150388324,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55598658,
            "range": "± 844915",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 374488962,
            "range": "± 969298",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29098685,
            "range": "± 386818",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1dca2803c66eb0d7aca33a4c8d1f2f81c2d80b26",
          "message": "Bump thread_local from 1.1.3 to 1.1.7 (#673)\n\nBumps [thread_local](https://github.com/Amanieu/thread_local-rs) from 1.1.3 to 1.1.7.\r\n- [Release notes](https://github.com/Amanieu/thread_local-rs/releases)\r\n- [Commits](https://github.com/Amanieu/thread_local-rs/compare/v1.1.3...1.1.7)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: thread_local\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:08:58+01:00",
          "tree_id": "2e2781a23102e69a41390ee31798ec3fbd237574",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1dca2803c66eb0d7aca33a4c8d1f2f81c2d80b26"
        },
        "date": 1681150438334,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 52555862,
            "range": "± 793797",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368272650,
            "range": "± 369085",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28429966,
            "range": "± 75841",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5b182d59993ded7bc167a4b322ca4e1bbb244787",
          "message": "Bump criterion from 0.3.5 to 0.4.0 (#676)\n\nBumps [criterion](https://github.com/bheisler/criterion.rs) from 0.3.5 to 0.4.0.\r\n- [Release notes](https://github.com/bheisler/criterion.rs/releases)\r\n- [Changelog](https://github.com/bheisler/criterion.rs/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/bheisler/criterion.rs/compare/0.3.5...0.4.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: criterion\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:10:19+01:00",
          "tree_id": "3e4e878d1225c5780a2ea840d802a352997c5127",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5b182d59993ded7bc167a4b322ca4e1bbb244787"
        },
        "date": 1681150478714,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54972895,
            "range": "± 684097",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369437924,
            "range": "± 3166717",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28602305,
            "range": "± 272182",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ddd49107bef047583bf374c17ac1d66e836f2643",
          "message": "Bump nanoid and mocha in /stylua-vscode (#671)\n\nBumps [nanoid](https://github.com/ai/nanoid) to 3.3.3 and updates ancestor dependency [mocha](https://github.com/mochajs/mocha). These dependencies need to be updated together.\r\n\r\n\r\nUpdates `nanoid` from 3.1.20 to 3.3.3\r\n- [Release notes](https://github.com/ai/nanoid/releases)\r\n- [Changelog](https://github.com/ai/nanoid/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/ai/nanoid/compare/3.1.20...3.3.3)\r\n\r\nUpdates `mocha` from 8.4.0 to 10.2.0\r\n- [Release notes](https://github.com/mochajs/mocha/releases)\r\n- [Changelog](https://github.com/mochajs/mocha/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/mochajs/mocha/compare/v8.4.0...v10.2.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: nanoid\r\n  dependency-type: indirect\r\n- dependency-name: mocha\r\n  dependency-type: direct:development\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-10T19:09:32+01:00",
          "tree_id": "7b1bc1899768e919d9df9e7ac4e8e55aaf5eb75f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ddd49107bef047583bf374c17ac1d66e836f2643"
        },
        "date": 1681150498935,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54105756,
            "range": "± 955187",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369316816,
            "range": "± 1211127",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28566247,
            "range": "± 85002",
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
          "id": "40eae81c023817cce0c91f96c4d6400d3512aae0",
          "message": "Disable CI mode for test cases update",
          "timestamp": "2023-04-17T14:42:48+01:00",
          "tree_id": "286fb230e8d57d33fc0b3231375547721b8673ea",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/40eae81c023817cce0c91f96c4d6400d3512aae0"
        },
        "date": 1681739247604,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 73939860,
            "range": "± 2306008",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 491078467,
            "range": "± 12825783",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 36939922,
            "range": "± 1799097",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8eec0864dc42244613e87a3aa0e320bc2aa58774",
          "message": "Bump anyhow from 1.0.53 to 1.0.70 (#683)\n\nBumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.53 to 1.0.70.\r\n- [Release notes](https://github.com/dtolnay/anyhow/releases)\r\n- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.53...1.0.70)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: anyhow\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-17T17:16:48+01:00",
          "tree_id": "9a5c448f763048dfb8230302ee3b28b1b6a13b55",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8eec0864dc42244613e87a3aa0e320bc2aa58774"
        },
        "date": 1681748484311,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 67068289,
            "range": "± 2781119",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 435730711,
            "range": "± 714014",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 36189613,
            "range": "± 432300",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "396ef88083c57c7371e553add38fcc9dc148081d",
          "message": "Bump thiserror from 1.0.31 to 1.0.40 (#686)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.31 to 1.0.40.\r\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\r\n- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.31...1.0.40)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: thiserror\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-17T17:17:12+01:00",
          "tree_id": "b85744806c90a30393f2cd21cfcc2a00573e6f5b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/396ef88083c57c7371e553add38fcc9dc148081d"
        },
        "date": 1681748502995,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 68770799,
            "range": "± 2426851",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 473256469,
            "range": "± 10889465",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 35070465,
            "range": "± 1015048",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fdaaa2b10364db8298821b83ae5a9bc28004ecf2",
          "message": "Bump globset from 0.4.8 to 0.4.10 (#685)\n\nBumps [globset](https://github.com/BurntSushi/ripgrep) from 0.4.8 to 0.4.10.\r\n- [Release notes](https://github.com/BurntSushi/ripgrep/releases)\r\n- [Changelog](https://github.com/BurntSushi/ripgrep/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/BurntSushi/ripgrep/compare/globset-0.4.8...ignore-0.4.10)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: globset\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-17T17:18:08+01:00",
          "tree_id": "d741f61b3fecf82f3525b935bf91fa1c135ad05f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fdaaa2b10364db8298821b83ae5a9bc28004ecf2"
        },
        "date": 1681748556198,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55969561,
            "range": "± 450791",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370512209,
            "range": "± 1159860",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28758016,
            "range": "± 99546",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9c906f1799bb93e1c9b74c5fa1b9576ce8b6261f",
          "message": "Bump console from 0.15.0 to 0.15.5 (#682)\n\nBumps [console](https://github.com/console-rs/console) from 0.15.0 to 0.15.5.\r\n- [Release notes](https://github.com/console-rs/console/releases)\r\n- [Changelog](https://github.com/console-rs/console/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/console-rs/console/compare/0.15.0...0.15.5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: console\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-17T17:23:32+01:00",
          "tree_id": "084709358a50108cfbc1c22bc82346bd511f8b1b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/9c906f1799bb93e1c9b74c5fa1b9576ce8b6261f"
        },
        "date": 1681748833755,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53473760,
            "range": "± 914854",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370875124,
            "range": "± 893203",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28005546,
            "range": "± 140586",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "36c691eff781e99e7125f366f1dd04de8c08fef3",
          "message": "Bump ec4rs from 1.0.1 to 1.0.2 (#684)",
          "timestamp": "2023-04-17T18:04:12+01:00",
          "tree_id": "765229079adf9ecc8331da78ea11abff648ee109",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/36c691eff781e99e7125f366f1dd04de8c08fef3"
        },
        "date": 1681751285530,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57188817,
            "range": "± 661516",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371367121,
            "range": "± 1258035",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30641434,
            "range": "± 1538494",
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
          "id": "57e002c0c14139754cfe573cef32acf1d2d6250a",
          "message": "Update external test cases (#681)",
          "timestamp": "2023-04-17T18:04:30+01:00",
          "tree_id": "e04828b265d35072cf258ba841a3bd0aea6820a1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/57e002c0c14139754cfe573cef32acf1d2d6250a"
        },
        "date": 1681751299864,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58693412,
            "range": "± 607365",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371489825,
            "range": "± 644124",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28907774,
            "range": "± 582523",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "91977cd5a201e98c092464cad3c74d1b4ed44a04",
          "message": "Bump num_cpus from 1.13.1 to 1.15.0 (#689)",
          "timestamp": "2023-04-24T17:13:55+01:00",
          "tree_id": "495942d167a204913a067349830d3d9679387282",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/91977cd5a201e98c092464cad3c74d1b4ed44a04"
        },
        "date": 1682353118737,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55461018,
            "range": "± 726441",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369257142,
            "range": "± 1086836",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29572927,
            "range": "± 161411",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b33795c62371e86a3805cad4ad34e513a1624dee",
          "message": "Bump env_logger from 0.9.0 to 0.10.0 (#690)",
          "timestamp": "2023-04-24T17:54:43+01:00",
          "tree_id": "57600541c8584744bddfefbc3b90f824b5e758ca",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b33795c62371e86a3805cad4ad34e513a1624dee"
        },
        "date": 1682355525143,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55853799,
            "range": "± 612979",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371231486,
            "range": "± 866741",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28562268,
            "range": "± 188945",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ed58d91c806985c7993c3f2489a17907e71dc8df",
          "message": "Bump crossbeam-channel from 0.5.4 to 0.5.8 (#688)",
          "timestamp": "2023-04-24T19:05:51+01:00",
          "tree_id": "914493199e66f273892acccabac3895855c0aca5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ed58d91c806985c7993c3f2489a17907e71dc8df"
        },
        "date": 1682359805673,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56816142,
            "range": "± 640478",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371978698,
            "range": "± 1473928",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28471561,
            "range": "± 511107",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0e1a5a8fefd32e32dbbbd0da25e3b756555dda85",
          "message": "Bump serde_json from 1.0.95 to 1.0.96 (#691)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.95 to 1.0.96.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.95...v1.0.96)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-24T19:42:32+01:00",
          "tree_id": "dca00e771b999a56f2f956fc4ad5eed0ade361ab",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0e1a5a8fefd32e32dbbbd0da25e3b756555dda85"
        },
        "date": 1682362004824,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55405428,
            "range": "± 1057285",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 371717837,
            "range": "± 575079",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29107809,
            "range": "± 346920",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "951e7ee433e55eb5cfd679682b7127b025d11041",
          "message": "Bump regex from 1.7.3 to 1.8.1 (#692)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.7.3 to 1.8.1.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.7.3...1.8.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-04-24T19:55:48+01:00",
          "tree_id": "40e091b75a108065d9ef949bfa0bcb00eed74671",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/951e7ee433e55eb5cfd679682b7127b025d11041"
        },
        "date": 1682362811210,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56077739,
            "range": "± 641632",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 372175233,
            "range": "± 1018737",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29052602,
            "range": "± 275180",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dbf72b94b3164149ad0d1cdd4a938131337f1e44",
          "message": "Bump serde from 1.0.136 to 1.0.160 (#697)\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.136 to 1.0.160.\r\n- [Release notes](https://github.com/serde-rs/serde/releases)\r\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.136...v1.0.160)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-01T23:20:02+01:00",
          "tree_id": "27419f54b97f72626a4e112bfbdcccb3a08adf84",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/dbf72b94b3164149ad0d1cdd4a938131337f1e44"
        },
        "date": 1682979853200,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55204642,
            "range": "± 749265",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369657612,
            "range": "± 1269698",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28232828,
            "range": "± 64385",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b8b702c0119ea4095ca009705c3637a7f3eb0d0a",
          "message": "Bump log from 0.4.14 to 0.4.17 (#696)\n\nBumps [log](https://github.com/rust-lang/log) from 0.4.14 to 0.4.17.\r\n- [Release notes](https://github.com/rust-lang/log/releases)\r\n- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/log/compare/0.4.14...0.4.17)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: log\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-01T23:26:09+01:00",
          "tree_id": "2fc3013d45182a5de9e12ba1c2e0c88f82b138d1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b8b702c0119ea4095ca009705c3637a7f3eb0d0a"
        },
        "date": 1682980227703,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53054798,
            "range": "± 278663",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368624179,
            "range": "± 1077513",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27841173,
            "range": "± 120200",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c8d51468502fc34735751dcc8364d3abc6bc9144",
          "message": "Bump toml from 0.5.8 to 0.5.11 (#695)\n\nBumps [toml](https://github.com/toml-rs/toml) from 0.5.8 to 0.5.11.\r\n- [Release notes](https://github.com/toml-rs/toml/releases)\r\n- [Commits](https://github.com/toml-rs/toml/commits/toml-v0.5.11)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: toml\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-05-01T23:26:44+01:00",
          "tree_id": "cb82a245bbdb6ce12e016e6b2f2ca39cfbd1f22e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c8d51468502fc34735751dcc8364d3abc6bc9144"
        },
        "date": 1682980260088,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55833249,
            "range": "± 547676",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 373719844,
            "range": "± 709207",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28348211,
            "range": "± 315103",
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
          "id": "a1163d6a96ef2204b22e5fb9f751ca4b5542b43e",
          "message": "Mention style guide StyLua follows\n\nCloses #693",
          "timestamp": "2023-06-07T12:42:40+01:00",
          "tree_id": "866945b5adc4dc6ec4af67299c53cfc1cc175703",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a1163d6a96ef2204b22e5fb9f751ca4b5542b43e"
        },
        "date": 1686138362751,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54654575,
            "range": "± 690465",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 372477400,
            "range": "± 2089000",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27937065,
            "range": "± 332548",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "73e64f6c2043493cace17b0a07468affd180550b",
          "message": "Bump anyhow from 1.0.70 to 1.0.71 (#699)\n\nBumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.70 to 1.0.71.\r\n- [Release notes](https://github.com/dtolnay/anyhow/releases)\r\n- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.70...1.0.71)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: anyhow\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-07T12:45:38+01:00",
          "tree_id": "490367a71bca51b7ca698b5a8efeecefaada6cc8",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/73e64f6c2043493cace17b0a07468affd180550b"
        },
        "date": 1686138639417,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55595437,
            "range": "± 703759",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370615510,
            "range": "± 1436065",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29700272,
            "range": "± 890981",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "61f411c96d8e78ac371ec38057435717b911e1a2",
          "message": "Bump serde from 1.0.160 to 1.0.163 (#702)\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.160 to 1.0.163.\r\n- [Release notes](https://github.com/serde-rs/serde/releases)\r\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.160...v1.0.163)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-07T12:47:30+01:00",
          "tree_id": "2dc2274cb87422e409fb7ac332fee5a8552f5bdf",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/61f411c96d8e78ac371ec38057435717b911e1a2"
        },
        "date": 1686138845317,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 78777213,
            "range": "± 3132030",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 529283811,
            "range": "± 8445205",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 40666147,
            "range": "± 1268601",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "16d167ab07539b1c39fa7ab6ce0b515633ca680e",
          "message": "Bump ignore from 0.4.18 to 0.4.20 (#700)\n\nBumps [ignore](https://github.com/BurntSushi/ripgrep) from 0.4.18 to 0.4.20.\r\n- [Release notes](https://github.com/BurntSushi/ripgrep/releases)\r\n- [Changelog](https://github.com/BurntSushi/ripgrep/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/BurntSushi/ripgrep/compare/ignore-0.4.18...ignore-0.4.20)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: ignore\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-07T12:53:24+01:00",
          "tree_id": "cab5056b2d70b98ff2925c865761a62ee845ca15",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/16d167ab07539b1c39fa7ab6ce0b515633ca680e"
        },
        "date": 1686139106408,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55709588,
            "range": "± 911931",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368518122,
            "range": "± 576948",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28324135,
            "range": "± 167458",
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
          "id": "53f5a698f06cec5ff9eacacf207a4335af374dc8",
          "message": "Check table brace for function args multiline (#706)\n\n* Check table brace for function args multiline\r\n\r\n* Update changelog",
          "timestamp": "2023-06-07T13:08:35+01:00",
          "tree_id": "3eb0d7a5fb1392b4b1e87425956948d35dd247d9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/53f5a698f06cec5ff9eacacf207a4335af374dc8"
        },
        "date": 1686139916336,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54469533,
            "range": "± 377533",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 374663646,
            "range": "± 1738879",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27930431,
            "range": "± 196797",
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
          "id": "d3659f3f91a53899c5a0e04a8b1e62f4702b2878",
          "message": "Support multiline ignores in table fields (#707)\n\n* Add test case\r\n\r\n* Support multiline ignores in table fields\r\n\r\n* Add snapshot\r\n\r\n* Update changelog",
          "timestamp": "2023-06-07T19:37:15+01:00",
          "tree_id": "cf382e46352a3b1bfa3e36b77bf4297a740bf4d3",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d3659f3f91a53899c5a0e04a8b1e62f4702b2878"
        },
        "date": 1686163399430,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66599359,
            "range": "± 444982",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 444049136,
            "range": "± 1070189",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 35984828,
            "range": "± 576212",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bf08e0a85fae4d719c2f23fc53923b796fc6be0a",
          "message": "Bump console from 0.15.5 to 0.15.7 (#709)\n\nBumps [console](https://github.com/console-rs/console) from 0.15.5 to 0.15.7.\r\n- [Changelog](https://github.com/console-rs/console/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/console-rs/console/compare/0.15.5...0.15.7)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: console\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-13T14:52:29+01:00",
          "tree_id": "8d8f5a9f9537d6da59b3e8e58c209134cfc7a74f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bf08e0a85fae4d719c2f23fc53923b796fc6be0a"
        },
        "date": 1686664741656,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 69955833,
            "range": "± 3724651",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 495313835,
            "range": "± 15411586",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 38067589,
            "range": "± 1753347",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "18b70a83127b604ce63994c9cc385059630f84ad",
          "message": "Bump serde from 1.0.163 to 1.0.164 (#711)\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.163 to 1.0.164.\r\n- [Release notes](https://github.com/serde-rs/serde/releases)\r\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.163...v1.0.164)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-13T14:56:15+01:00",
          "tree_id": "0df43b798234b917b0b57764442b80571c062343",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/18b70a83127b604ce63994c9cc385059630f84ad"
        },
        "date": 1686664886644,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56037576,
            "range": "± 382778",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 379231178,
            "range": "± 2054897",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28859214,
            "range": "± 133037",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fad3f1706aa683b1bca63d611d203a0578ed78ad",
          "message": "Bump log from 0.4.17 to 0.4.19 (#710)\n\nBumps [log](https://github.com/rust-lang/log) from 0.4.17 to 0.4.19.\r\n- [Release notes](https://github.com/rust-lang/log/releases)\r\n- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/log/compare/0.4.17...0.4.19)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: log\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-13T15:11:19+01:00",
          "tree_id": "135512139d1a2db461813596f342718a7e60d086",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fad3f1706aa683b1bca63d611d203a0578ed78ad"
        },
        "date": 1686665851636,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 63979662,
            "range": "± 3846120",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 474531093,
            "range": "± 14384854",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 36221743,
            "range": "± 2047550",
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
          "id": "b12c4bf74ce78cf7724013fa397effafb704fc7a",
          "message": "Don't remove parentheses around optional type (#680)\n\n* Add test case\r\n\r\n* Keep parens around optional\r\n\r\n* Update changelog\r\n\r\n* Improve test suite\r\n\r\n* Pass arround context for intelligent excess parens removal\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2023-06-14T14:02:57+01:00",
          "tree_id": "02811724206d16bcfec710ae0077fdbb7bf41ec2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b12c4bf74ce78cf7724013fa397effafb704fc7a"
        },
        "date": 1686748098310,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59262854,
            "range": "± 383659",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 381503804,
            "range": "± 840975",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30464971,
            "range": "± 338930",
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
          "id": "5c6d59135f419274121349799a5227be467358b1",
          "message": "Fix race condition when same file formatted more than once (#712)",
          "timestamp": "2023-06-14T14:32:36+01:00",
          "tree_id": "43f0f4093ba459965626acdff92cbda7948aedec",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5c6d59135f419274121349799a5227be467358b1"
        },
        "date": 1686749865380,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54824265,
            "range": "± 644312",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 381870195,
            "range": "± 579388",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29481992,
            "range": "± 150250",
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
          "id": "40c1b0826d56d88ef28be13ef0f86fec04415f26",
          "message": "Add option `call_parentheses = \"Input\"` (#713)\n\n* Add input option\r\n\r\n* Rely on input for call parentheses\r\n\r\n* Add test, update changelog and readme",
          "timestamp": "2023-06-14T15:04:40+01:00",
          "tree_id": "be0f2a23394406b4682a6fcfc34298a6918c30de",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/40c1b0826d56d88ef28be13ef0f86fec04415f26"
        },
        "date": 1686751798877,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58130840,
            "range": "± 546195",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 384915478,
            "range": "± 989762",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31865523,
            "range": "± 331977",
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
          "id": "2499d711b896f26e77d3b7599f185ccf700b61cd",
          "message": "v0.18.0",
          "timestamp": "2023-06-14T16:04:45+01:00",
          "tree_id": "35a8ac86770cdabcd0a5d18123133fb9662fcd04",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2499d711b896f26e77d3b7599f185ccf700b61cd"
        },
        "date": 1686755337952,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58981823,
            "range": "± 548458",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 378938352,
            "range": "± 1084365",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29155729,
            "range": "± 555613",
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
          "id": "0d4decb462b2eb84d5e7ef1f8feba39c9890bdae",
          "message": "Bump ubuntu runner versions",
          "timestamp": "2023-06-14T16:15:13+01:00",
          "tree_id": "3c32828faf9524384c832a500affe033853a82c2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0d4decb462b2eb84d5e7ef1f8feba39c9890bdae"
        },
        "date": 1686756059706,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58140792,
            "range": "± 395421",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 387408399,
            "range": "± 2292912",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31647369,
            "range": "± 316590",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64d3032edd0e400ff866278c3f3118446b42a7af",
          "message": "Bump serde_json from 1.0.96 to 1.0.97 (#720)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.96 to 1.0.97.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.96...v1.0.97)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-19T18:03:50+01:00",
          "tree_id": "4cef97aa6e8409d39861d33b02624ea4b7db920e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/64d3032edd0e400ff866278c3f3118446b42a7af"
        },
        "date": 1687194439482,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 52710746,
            "range": "± 565166",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 377605409,
            "range": "± 1002694",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28506676,
            "range": "± 76273",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "353646bbfe08c4495661db6914ce6face339009c",
          "message": "Bump strum from 0.24.1 to 0.25.0 (#719)\n\nBumps [strum](https://github.com/Peternator7/strum) from 0.24.1 to 0.25.0.\r\n- [Changelog](https://github.com/Peternator7/strum/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/Peternator7/strum/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: strum\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-19T18:17:06+01:00",
          "tree_id": "43c2d932cb06e5eef72b74aacd39e2a072d89d6d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/353646bbfe08c4495661db6914ce6face339009c"
        },
        "date": 1687195225998,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55792691,
            "range": "± 594422",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 382032843,
            "range": "± 1813838",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29331703,
            "range": "± 131977",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9d67f9818e831bbb1a8fbb5314415a2a84dd8712",
          "message": "Bump similar from 2.1.0 to 2.2.1 (#721)\n\nBumps [similar](https://github.com/mitsuhiko/similar) from 2.1.0 to 2.2.1.\r\n- [Changelog](https://github.com/mitsuhiko/similar/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/mitsuhiko/similar/compare/2.1.0...2.2.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: similar\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-19T19:30:48+01:00",
          "tree_id": "54aa9dfaeae240a908fca678eeed2e3c75214c14",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/9d67f9818e831bbb1a8fbb5314415a2a84dd8712"
        },
        "date": 1687199643317,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56988317,
            "range": "± 637148",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 376671941,
            "range": "± 1097477",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28881745,
            "range": "± 163194",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2b5cbfce6d027e31d2a7e6b794c7efbd844d1d7b",
          "message": "Bump serde_json from 1.0.97 to 1.0.99 (#723)",
          "timestamp": "2023-06-26T17:14:25+01:00",
          "tree_id": "ddcee4a6d6b0900abeeb022602d786c507500ca1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2b5cbfce6d027e31d2a7e6b794c7efbd844d1d7b"
        },
        "date": 1687796298499,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 62895647,
            "range": "± 3826774",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 443538154,
            "range": "± 9144932",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34211244,
            "range": "± 1241410",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a18f9a62c36f24a6a31f4bf552610d77e245830c",
          "message": "Bump regex from 1.8.1 to 1.8.4 (#725)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.8.1 to 1.8.4.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.8.1...1.8.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-27T13:26:19+01:00",
          "tree_id": "ab4972d8bf43bfd2be03fd8e65218d573355fc86",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a18f9a62c36f24a6a31f4bf552610d77e245830c"
        },
        "date": 1687868987182,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54378833,
            "range": "± 458891",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 379394899,
            "range": "± 1343925",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28651206,
            "range": "± 171533",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dfcc7c0fcbd82e137fc941104345fa3ffd53b147",
          "message": "Bump serde from 1.0.164 to 1.0.165 (#728)\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.164 to 1.0.165.\r\n- [Release notes](https://github.com/serde-rs/serde/releases)\r\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.164...v1.0.165)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-03T15:27:42+01:00",
          "tree_id": "2bc8d9fae2b0deba126a7af2f6551054f505d65d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/dfcc7c0fcbd82e137fc941104345fa3ffd53b147"
        },
        "date": 1688394678210,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58304012,
            "range": "± 348782",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 389834845,
            "range": "± 2599086",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31347987,
            "range": "± 334870",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8a56233089851f27fb1ded6acdfbc92b80a13260",
          "message": "Bump num_cpus from 1.15.0 to 1.16.0 (#727)\n\nBumps [num_cpus](https://github.com/seanmonstar/num_cpus) from 1.15.0 to 1.16.0.\r\n- [Release notes](https://github.com/seanmonstar/num_cpus/releases)\r\n- [Changelog](https://github.com/seanmonstar/num_cpus/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/seanmonstar/num_cpus/compare/v1.15.0...v1.16.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: num_cpus\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-03T15:42:31+01:00",
          "tree_id": "9c669a93b1ec56955cc5e53508fdfd639c2b052f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8a56233089851f27fb1ded6acdfbc92b80a13260"
        },
        "date": 1688395546779,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55202657,
            "range": "± 491164",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 382721614,
            "range": "± 588059",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29304024,
            "range": "± 273512",
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
          "id": "05c4491a1448901d7af43957e12ed0094d267fb2",
          "message": "Add links to changelog items",
          "timestamp": "2023-07-06T11:45:44+01:00",
          "tree_id": "8738c68bc6c11a2f7f4e28e061ae632f5dc90acd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/05c4491a1448901d7af43957e12ed0094d267fb2"
        },
        "date": 1688640587338,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 70248497,
            "range": "± 3795854",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 475282216,
            "range": "± 19419364",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34479316,
            "range": "± 2212452",
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
          "id": "0e2c60d46a3e62924370eb865bcee3f5d8b5b027",
          "message": "Keep type pack parentheses in generic (#730)\n\n* Add test case\r\n\r\n* Keep parentheses within generics\r\n\r\n* Update snapshot and changelog",
          "timestamp": "2023-07-06T11:46:35+01:00",
          "tree_id": "929df81bed069e8bc26f5f76171afade31134380",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0e2c60d46a3e62924370eb865bcee3f5d8b5b027"
        },
        "date": 1688640603501,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58420185,
            "range": "± 389646",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 382845710,
            "range": "± 2811206",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30417032,
            "range": "± 540734",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "adb2a53450fb8bdd7a301b0f392057d61fecd06a",
          "message": "Bump toml from 0.5.11 to 0.7.5 (#724)\n\nBumps [toml](https://github.com/toml-rs/toml) from 0.5.11 to 0.7.5.\r\n- [Commits](https://github.com/toml-rs/toml/compare/toml-v0.5.11...toml-v0.7.5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: toml\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-06T11:46:03+01:00",
          "tree_id": "54ebfa16087bbe657ae2881413a92820bddf7cf6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/adb2a53450fb8bdd7a301b0f392057d61fecd06a"
        },
        "date": 1688640617830,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66614008,
            "range": "± 562213",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 448310247,
            "range": "± 1832462",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 36403283,
            "range": "± 395942",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f394e7928f4e9e67d754fbf5c02b18e358270495",
          "message": "Bump serde_json from 1.0.99 to 1.0.100 (#732)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.99 to 1.0.100.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.99...v1.0.100)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-11T12:12:49+01:00",
          "tree_id": "9d34a1c6a015c06b6202157cd56b5346eafa8ab9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f394e7928f4e9e67d754fbf5c02b18e358270495"
        },
        "date": 1689074184829,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56092481,
            "range": "± 1068723",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 380904630,
            "range": "± 2027952",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29253763,
            "range": "± 162917",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cf89af90fffbb942d1a5c95550c77d49482b7b6b",
          "message": "Bump regex from 1.8.4 to 1.9.1 (#734)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.8.4 to 1.9.1.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.8.4...1.9.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-15T19:54:29+01:00",
          "tree_id": "f7fff6a59462b3760b93e91049598b112bb44ae7",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/cf89af90fffbb942d1a5c95550c77d49482b7b6b"
        },
        "date": 1689447486491,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54730626,
            "range": "± 474114",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 380534551,
            "range": "± 4135783",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29438170,
            "range": "± 112215",
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
          "id": "0fb1c1532957d229c59cb477232677fe7220de2f",
          "message": "v0.18.1",
          "timestamp": "2023-07-15T20:12:45+01:00",
          "tree_id": "09a3684def54ba2e8e1346631f365eeb0af973d7",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0fb1c1532957d229c59cb477232677fe7220de2f"
        },
        "date": 1689448609979,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55910727,
            "range": "± 752721",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 380031648,
            "range": "± 1099322",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29661225,
            "range": "± 239535",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fdf9f0c5952cb1565ba6449929514eb94ac5ef3f",
          "message": "Bump word-wrap from 1.2.3 to 1.2.4 in /stylua-vscode (#740)\n\nBumps [word-wrap](https://github.com/jonschlinkert/word-wrap) from 1.2.3 to 1.2.4.\r\n- [Release notes](https://github.com/jonschlinkert/word-wrap/releases)\r\n- [Commits](https://github.com/jonschlinkert/word-wrap/compare/1.2.3...1.2.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: word-wrap\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-01T19:44:44+01:00",
          "tree_id": "c437d1a1688e9bb65cab2bb1348e0380b4fb1769",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fdf9f0c5952cb1565ba6449929514eb94ac5ef3f"
        },
        "date": 1690915695635,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55595219,
            "range": "± 638799",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 375612600,
            "range": "± 1525245",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28710371,
            "range": "± 277561",
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
          "id": "fb5a2776875707726cee6042e82b779c73ee50a3",
          "message": "Fix LuaJIT number panics in verify mode (#752)\n\n* Fix LuaJIT number panics in verify mode\r\n\r\n* Update changelog",
          "timestamp": "2023-09-01T19:44:02-05:00",
          "tree_id": "43bfb07c52e508e2c2ababaec8dd56d6bab1382b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fb5a2776875707726cee6042e82b779c73ee50a3"
        },
        "date": 1693615654426,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56315270,
            "range": "± 384909",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 377807048,
            "range": "± 575200",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28929068,
            "range": "± 163646",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d4e680b8143a0aacbce55619f296711f579844c6",
          "message": "Bump thiserror from 1.0.40 to 1.0.47 (#746)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.40 to 1.0.47.\r\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\r\n- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.40...1.0.47)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: thiserror\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-01T19:49:18-05:00",
          "tree_id": "8e1415e15bff4f7cbc34ef102958995a9b711047",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d4e680b8143a0aacbce55619f296711f579844c6"
        },
        "date": 1693615956286,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53370130,
            "range": "± 413473",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 374778234,
            "range": "± 1246381",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28164685,
            "range": "± 59778",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b170290cfd0d725c5282797409dcca77d10af40c",
          "message": "Bump anyhow from 1.0.71 to 1.0.75 (#745)\n\nBumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.71 to 1.0.75.\r\n- [Release notes](https://github.com/dtolnay/anyhow/releases)\r\n- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.71...1.0.75)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: anyhow\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-01T19:49:42-05:00",
          "tree_id": "fdb91263d1db6cfa72fb30254c1abe9f8ef588ac",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b170290cfd0d725c5282797409dcca77d10af40c"
        },
        "date": 1693616001110,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58648147,
            "range": "± 197900",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 394640138,
            "range": "± 903376",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 32181262,
            "range": "± 121060",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d04e7f3e9c40cce5cea842b8aaa334cefbc6a73b",
          "message": "Bump toml from 0.7.5 to 0.7.6 (#737)\n\nBumps [toml](https://github.com/toml-rs/toml) from 0.7.5 to 0.7.6.\r\n- [Commits](https://github.com/toml-rs/toml/compare/toml-v0.7.5...toml-v0.7.6)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: toml\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-01T19:49:59-05:00",
          "tree_id": "3d5de4658e220068a799e294950b5f98e3bed42d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d04e7f3e9c40cce5cea842b8aaa334cefbc6a73b"
        },
        "date": 1693616028027,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57848643,
            "range": "± 3044445",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 402778058,
            "range": "± 14883429",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30949735,
            "range": "± 1009135",
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
          "id": "a0d1b5ddeb1d4a24f43dc19f4cea7d2a917df87d",
          "message": "Fix collapsed if stmt formatting when block begins with empty line (#753)\n\n* Add test case\r\n\r\n* Fix code\r\n\r\n* Update snapshot and changelog",
          "timestamp": "2023-09-01T19:50:10-05:00",
          "tree_id": "d7244e220bcea3a34fec345b610cd3ce6f752b82",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a0d1b5ddeb1d4a24f43dc19f4cea7d2a917df87d"
        },
        "date": 1693616039204,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56456350,
            "range": "± 2701651",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 380205465,
            "range": "± 2599074",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28827612,
            "range": "± 119229",
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
          "id": "dd97aa341c66e5284185a41eae77b58f4d1c2d7e",
          "message": "Fix dot func call chains with comment after dot (#754)\n\n* Add test case\r\n\r\n* Fix code\r\n\r\n* Update snapshot and changelog",
          "timestamp": "2023-09-01T19:56:52-05:00",
          "tree_id": "681667eda68e1fb679464d9ef4bd688f217d8b1e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/dd97aa341c66e5284185a41eae77b58f4d1c2d7e"
        },
        "date": 1693616447144,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 64255623,
            "range": "± 1194414",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 438937124,
            "range": "± 6240054",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 33956725,
            "range": "± 428534",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "21e82a282c28e6a86cc71e074c4dc8c8d3b4b6e5",
          "message": "Bump serde from 1.0.171 to 1.0.179 (#757)",
          "timestamp": "2023-09-06T12:45:23-05:00",
          "tree_id": "b3beca873effcc16e56dbc4ffd320d2afb8872c4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/21e82a282c28e6a86cc71e074c4dc8c8d3b4b6e5"
        },
        "date": 1694022537521,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54637116,
            "range": "± 544565",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 378545042,
            "range": "± 1027743",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28913556,
            "range": "± 199894",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d2b13f4f93b5eb20b9adb35ef8250afebf7aec6a",
          "message": "Bump serde_json from 1.0.100 to 1.0.105 (#756)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.100 to 1.0.105.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.100...v1.0.105)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-09T11:20:08-05:00",
          "tree_id": "c24fdb13f39664b4cfbd767583fb8dfba1fffae3",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d2b13f4f93b5eb20b9adb35ef8250afebf7aec6a"
        },
        "date": 1694276619804,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56421394,
            "range": "± 540072",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 390510569,
            "range": "± 589455",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29660338,
            "range": "± 193746",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0898917692d944ebcbaa16c17cba56fe5e8abe42",
          "message": "Bump thiserror from 1.0.47 to 1.0.48 (#758)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.47 to 1.0.48.\r\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\r\n- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.47...1.0.48)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: thiserror\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-09T11:20:30-05:00",
          "tree_id": "5522f76869b31805ef2daf1edc717ec2cfab0551",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0898917692d944ebcbaa16c17cba56fe5e8abe42"
        },
        "date": 1694276643638,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59398504,
            "range": "± 477091",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 399990090,
            "range": "± 1176355",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31850137,
            "range": "± 429216",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "607e9c80eae4189de3c0751e7bed313a0fa66551",
          "message": "Bump actions/checkout from 3 to 4 (#759)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 3 to 4.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v3...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-10T11:44:43-05:00",
          "tree_id": "86b8199cb5ba6aabfe12733e6ef1279065aa7077",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/607e9c80eae4189de3c0751e7bed313a0fa66551"
        },
        "date": 1694364487205,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59374073,
            "range": "± 549011",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 396449606,
            "range": "± 443087",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31391180,
            "range": "± 578452",
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
          "id": "5a1a3c992aef5544dd0d73b7472cdc48c78b1df2",
          "message": "Fix clippy warning",
          "timestamp": "2023-09-10T11:55:00-05:00",
          "tree_id": "3a128776e6046a137912624233ef2c92fa4fdde8",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5a1a3c992aef5544dd0d73b7472cdc48c78b1df2"
        },
        "date": 1694365105957,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57430722,
            "range": "± 339131",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 397287981,
            "range": "± 550338",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31120840,
            "range": "± 223774",
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
          "id": "ca8f419eb6d9c43645d3d097d33538b690e30523",
          "message": "v0.18.2",
          "timestamp": "2023-09-10T12:01:40-05:00",
          "tree_id": "0c4e65adefb67f432aa6435909d6954401b6c2de",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ca8f419eb6d9c43645d3d097d33538b690e30523"
        },
        "date": 1694365518098,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55968719,
            "range": "± 666631",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 393416329,
            "range": "± 842768",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30268202,
            "range": "± 465182",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "990e30c1f925ef87bce5d353f6147fc326314c60",
          "message": "Bump globset from 0.4.10 to 0.4.13 (#764)\n\nBumps [globset](https://github.com/BurntSushi/ripgrep) from 0.4.10 to 0.4.13.\r\n- [Release notes](https://github.com/BurntSushi/ripgrep/releases)\r\n- [Changelog](https://github.com/BurntSushi/ripgrep/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/BurntSushi/ripgrep/compare/globset-0.4.10...globset-0.4.13)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: globset\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-12T12:58:44-05:00",
          "tree_id": "d9e52755e6b8f444d621ffe3bfeccd7639146a57",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/990e30c1f925ef87bce5d353f6147fc326314c60"
        },
        "date": 1694541744904,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 58058570,
            "range": "± 1092813",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 391618903,
            "range": "± 3614663",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30253719,
            "range": "± 171349",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cd38ec3b2bd1d2149090e0dfc3acf40eb32b6d4c",
          "message": "Bump serde from 1.0.179 to 1.0.188 (#763)",
          "timestamp": "2023-09-12T22:03:53-05:00",
          "tree_id": "3e18c1765aeeb092cfaf42bec003d0fc1a2f94ad",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/cd38ec3b2bd1d2149090e0dfc3acf40eb32b6d4c"
        },
        "date": 1694574450120,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57518498,
            "range": "± 294774",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 390215314,
            "range": "± 3081950",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29356373,
            "range": "± 153071",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1fcdc6e25750045742a25ffbf677de6f06c50fa3",
          "message": "Bump regex from 1.9.1 to 1.9.5 (#762)",
          "timestamp": "2023-09-14T08:38:41-05:00",
          "tree_id": "2d9027b56ff6fcb03c41956bdec5c337c89b9ac5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1fcdc6e25750045742a25ffbf677de6f06c50fa3"
        },
        "date": 1694698943208,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 60043478,
            "range": "± 507288",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 394313405,
            "range": "± 1569506",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30669901,
            "range": "± 296313",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4c99cc8fc46729c0ab8978cc7b7cafc2137d7da8",
          "message": "Bump insta from 1.12.0 to 1.30.0 (#722)",
          "timestamp": "2023-09-17T19:17:02-05:00",
          "tree_id": "3d267ca1a000ecdd00275e3b8259f4a18cda26a3",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4c99cc8fc46729c0ab8978cc7b7cafc2137d7da8"
        },
        "date": 1694996465245,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56993135,
            "range": "± 1089707",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 391364703,
            "range": "± 1787703",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30276705,
            "range": "± 181176",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ddee739eed6b2e39b39a579cfa4a5e474093be28",
          "message": "Bump serde_json from 1.0.105 to 1.0.107 (#772)",
          "timestamp": "2023-09-18T13:34:53-05:00",
          "tree_id": "464d124e2fc39242986e711a4bfc8dd9c5834e44",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ddee739eed6b2e39b39a579cfa4a5e474093be28"
        },
        "date": 1695062298000,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 60421891,
            "range": "± 693058",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 395274745,
            "range": "± 2174591",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31790037,
            "range": "± 537116",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "41db4f07998c826fe608069abb8d5b1c78b39e5c",
          "message": "Bump log from 0.4.19 to 0.4.20 (#774)\n\nBumps [log](https://github.com/rust-lang/log) from 0.4.19 to 0.4.20.\r\n- [Release notes](https://github.com/rust-lang/log/releases)\r\n- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/log/compare/0.4.19...0.4.20)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: log\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-21T19:33:44-05:00",
          "tree_id": "a3631128baed57a1144e2c81b1b71a0c38fe4e27",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/41db4f07998c826fe608069abb8d5b1c78b39e5c"
        },
        "date": 1695343033204,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57284518,
            "range": "± 1017542",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 394976517,
            "range": "± 1696819",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29772015,
            "range": "± 161059",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ad3e6a30192ad04ca30b9585068d1239e5e53604",
          "message": "Bump assert_cmd from 2.0.4 to 2.0.12 (#775)\n\nBumps [assert_cmd](https://github.com/assert-rs/assert_cmd) from 2.0.4 to 2.0.12.\r\n- [Changelog](https://github.com/assert-rs/assert_cmd/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/assert-rs/assert_cmd/compare/v2.0.4...v2.0.12)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: assert_cmd\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-21T19:34:03-05:00",
          "tree_id": "ed5c7cb5dc7dfa7911ccd2a658603d75cf490e7e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ad3e6a30192ad04ca30b9585068d1239e5e53604"
        },
        "date": 1695343096546,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 72933879,
            "range": "± 2798460",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 491234216,
            "range": "± 10629946",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 37577583,
            "range": "± 859881",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0404b73bf93360d0307de517b9333b400e599159",
          "message": "Bump toml from 0.7.6 to 0.8.0 (#781)\n\nBumps [toml](https://github.com/toml-rs/toml) from 0.7.6 to 0.8.0.\r\n- [Commits](https://github.com/toml-rs/toml/compare/toml-v0.7.6...toml-v0.8.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: toml\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-30T15:05:47+02:00",
          "tree_id": "c7ef76fc319a374303f10ec5432e127599076251",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0404b73bf93360d0307de517b9333b400e599159"
        },
        "date": 1696079398992,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66458684,
            "range": "± 459071",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 443043748,
            "range": "± 2630828",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34622710,
            "range": "± 240192",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "98bb478566c9354f1af6625dbd6a70e8ec0a6fbe",
          "message": "Bump insta from 1.30.0 to 1.32.0 (#780)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.30.0 to 1.32.0.\r\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.30.0...1.32.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: insta\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-30T15:17:26+02:00",
          "tree_id": "12f0691d7fa49198902084797c90667385611b19",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/98bb478566c9354f1af6625dbd6a70e8ec0a6fbe"
        },
        "date": 1696080064776,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59464590,
            "range": "± 447500",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 388093293,
            "range": "± 1758065",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31889825,
            "range": "± 700461",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "59885141+BBboy01@users.noreply.github.com",
            "name": "HyBer",
            "username": "BBboy01"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ad1cb871e08f7b15599395a2051c93b7200fd366",
          "message": "Update dependabot npm ecosystem configuration (#784)\n\nUpdate dependabot.yml",
          "timestamp": "2023-10-01T15:42:38+02:00",
          "tree_id": "45500f1723aa42cd3c3d0a67f2c3ba063e9a6e8f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ad1cb871e08f7b15599395a2051c93b7200fd366"
        },
        "date": 1696167976981,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 59742586,
            "range": "± 701703",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 384229942,
            "range": "± 3027516",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30646883,
            "range": "± 714774",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "083d25eebeb77b3106d829ea6a9db975781c13be",
          "message": "Bump unzipper from 0.10.11 to 0.10.14 in /stylua-npm-bin (#798)\n\nBumps [unzipper](https://github.com/ZJONSSON/node-unzipper) from 0.10.11 to 0.10.14.\r\n- [Release notes](https://github.com/ZJONSSON/node-unzipper/releases)\r\n- [Commits](https://github.com/ZJONSSON/node-unzipper/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: unzipper\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:30:18+02:00",
          "tree_id": "6a7814d1f13da1e4bd8a199a364c04cadc1e99c7",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/083d25eebeb77b3106d829ea6a9db975781c13be"
        },
        "date": 1696170819466,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56182931,
            "range": "± 629709",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 379526032,
            "range": "± 1556365",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 30137324,
            "range": "± 595460",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "465ddd9d6d399c86517826d5a46c9ba421412e40",
          "message": "Bump insta from 1.32.0 to 1.33.0 (#785)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.32.0 to 1.33.0.\r\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.32.0...1.33.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: insta\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:31:11+02:00",
          "tree_id": "2b8d7ed335d7c86bd42996985b5643d8a7acb230",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/465ddd9d6d399c86517826d5a46c9ba421412e40"
        },
        "date": 1696170872341,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55249814,
            "range": "± 437314",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 377813722,
            "range": "± 1584207",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28301151,
            "range": "± 91104",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8cf3e3f82e40a03908dfe6fa8f63ccbdde02adac",
          "message": "Bump thiserror from 1.0.48 to 1.0.49 (#786)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.48 to 1.0.49.\r\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\r\n- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.48...1.0.49)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: thiserror\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:31:25+02:00",
          "tree_id": "e03fcf2e6146f192ddd756e434f9afad710c4e82",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8cf3e3f82e40a03908dfe6fa8f63ccbdde02adac"
        },
        "date": 1696170903996,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 55091240,
            "range": "± 1037777",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 383133945,
            "range": "± 2221220",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29379117,
            "range": "± 247584",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7db6cc441539e8333f53813537658415ff86232d",
          "message": "Bump regex from 1.9.5 to 1.9.6 (#789)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.9.5 to 1.9.6.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.9.5...1.9.6)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:31:40+02:00",
          "tree_id": "788b673815438ec5dc57a689cc32f78a113607b5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7db6cc441539e8333f53813537658415ff86232d"
        },
        "date": 1696171000410,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57431197,
            "range": "± 941938",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 377548941,
            "range": "± 1175987",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28568223,
            "range": "± 235207",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "494c8cfd6414da5e564a1cd6622400414c769048",
          "message": "Bump @types/vscode from 1.53.0 to 1.82.0 in /stylua-vscode (#792)\n\nBumps [@types/vscode](https://github.com/DefinitelyTyped/DefinitelyTyped/tree/HEAD/types/vscode) from 1.53.0 to 1.82.0.\r\n- [Release notes](https://github.com/DefinitelyTyped/DefinitelyTyped/releases)\r\n- [Commits](https://github.com/DefinitelyTyped/DefinitelyTyped/commits/HEAD/types/vscode)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@types/vscode\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:33:44+02:00",
          "tree_id": "3b88de51ebe4bcce72c8a798d3135f2aa4b296a6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/494c8cfd6414da5e564a1cd6622400414c769048"
        },
        "date": 1696171041114,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 52676446,
            "range": "± 367151",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 376365974,
            "range": "± 957633",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27985175,
            "range": "± 44321",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eca6ccffb2111d8be28d1b6c942206d0132415ad",
          "message": "Bump node-fetch from 3.2.10 to 3.3.2 in /stylua-npm-bin (#788)\n\nBumps [node-fetch](https://github.com/node-fetch/node-fetch) from 3.2.10 to 3.3.2.\r\n- [Release notes](https://github.com/node-fetch/node-fetch/releases)\r\n- [Commits](https://github.com/node-fetch/node-fetch/compare/v3.2.10...v3.3.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: node-fetch\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:34:03+02:00",
          "tree_id": "31606b7b2904a969936be4aa030033c0932646dd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/eca6ccffb2111d8be28d1b6c942206d0132415ad"
        },
        "date": 1696171101679,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54119029,
            "range": "± 288104",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 375962642,
            "range": "± 1046506",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28058659,
            "range": "± 161942",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8d43b932528fa527fc0603cf1c918f3edf908e19",
          "message": "Bump vscode-test from 1.5.1 to 1.6.1 in /stylua-vscode (#797)\n\nBumps [vscode-test](https://github.com/Microsoft/vscode-test) from 1.5.1 to 1.6.1.\r\n- [Changelog](https://github.com/microsoft/vscode-test/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/Microsoft/vscode-test/compare/v1.5.1...v1.6.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: vscode-test\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:34:24+02:00",
          "tree_id": "33362b3a5aca6640a31ede2e4c187b13551f40ce",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8d43b932528fa527fc0603cf1c918f3edf908e19"
        },
        "date": 1696171165260,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54975907,
            "range": "± 454339",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 377967159,
            "range": "± 1561763",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28131062,
            "range": "± 153107",
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
          "id": "65e6f7bf979b8b0e3bf61a3523f02828524c1d0e",
          "message": "Add testing workflow for stylua-npm-bin (#799)",
          "timestamp": "2023-10-01T16:42:22+02:00",
          "tree_id": "433abac4e3fd44ed0bcb0735e1694d1461012b95",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/65e6f7bf979b8b0e3bf61a3523f02828524c1d0e"
        },
        "date": 1696171575427,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66006839,
            "range": "± 661738",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 442228949,
            "range": "± 2344290",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34746807,
            "range": "± 976520",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3a68bb8decd636f1eee45748fe28a100994b4775",
          "message": "Bump toml from 0.8.0 to 0.8.1 (#794)\n\nBumps [toml](https://github.com/toml-rs/toml) from 0.8.0 to 0.8.1.\r\n- [Commits](https://github.com/toml-rs/toml/compare/toml-v0.8.0...toml-v0.8.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: toml\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:43:16+02:00",
          "tree_id": "a5e2ebd75e437f2a6f21fff4ceeea33a72c470ad",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3a68bb8decd636f1eee45748fe28a100994b4775"
        },
        "date": 1696171660430,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 68184854,
            "range": "± 2446432",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 495834674,
            "range": "± 12337702",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 35826547,
            "range": "± 995035",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1d14f3d79b03931ecbfadcf36efd1d119b411253",
          "message": "Bump unzipper from 0.10.11 to 0.10.14 in /stylua-vscode (#790)\n\nBumps [unzipper](https://github.com/ZJONSSON/node-unzipper) from 0.10.11 to 0.10.14.\r\n- [Release notes](https://github.com/ZJONSSON/node-unzipper/releases)\r\n- [Commits](https://github.com/ZJONSSON/node-unzipper/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: unzipper\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:44:02+02:00",
          "tree_id": "54a74ecee38fe83aa04f086c1c9e00180cf905ae",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1d14f3d79b03931ecbfadcf36efd1d119b411253"
        },
        "date": 1696171698835,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 65169158,
            "range": "± 604400",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 443693215,
            "range": "± 3442464",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34364428,
            "range": "± 449012",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "79f1655156f7d73aeb5b02e02a2034b0e77447a6",
          "message": "Bump docker/login-action from 2 to 3 (#768)\n\nBumps [docker/login-action](https://github.com/docker/login-action) from 2 to 3.\r\n- [Release notes](https://github.com/docker/login-action/releases)\r\n- [Commits](https://github.com/docker/login-action/compare/v2...v3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: docker/login-action\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:45:25+02:00",
          "tree_id": "70fc83c8df406596411d0e76423e4d850f71ca1a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/79f1655156f7d73aeb5b02e02a2034b0e77447a6"
        },
        "date": 1696171779429,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 52620085,
            "range": "± 808203",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 375467977,
            "range": "± 719403",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27927422,
            "range": "± 52485",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "163838963545905c1e2c46424821d46e93f05b01",
          "message": "Bump docker/setup-buildx-action from 2 to 3 (#769)\n\nBumps [docker/setup-buildx-action](https://github.com/docker/setup-buildx-action) from 2 to 3.\r\n- [Release notes](https://github.com/docker/setup-buildx-action/releases)\r\n- [Commits](https://github.com/docker/setup-buildx-action/compare/v2...v3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: docker/setup-buildx-action\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:45:36+02:00",
          "tree_id": "05a097193d3ac1d47289ce8f0abce92846a86f0c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/163838963545905c1e2c46424821d46e93f05b01"
        },
        "date": 1696171815339,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54709085,
            "range": "± 751941",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 381072572,
            "range": "± 1137681",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28809339,
            "range": "± 145688",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1153999d09f2642f92378029c17e7207f86b76c6",
          "message": "Bump docker/metadata-action from 4 to 5 (#770)\n\nBumps [docker/metadata-action](https://github.com/docker/metadata-action) from 4 to 5.\r\n- [Release notes](https://github.com/docker/metadata-action/releases)\r\n- [Upgrade guide](https://github.com/docker/metadata-action/blob/master/UPGRADE.md)\r\n- [Commits](https://github.com/docker/metadata-action/compare/v4...v5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: docker/metadata-action\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:46:04+02:00",
          "tree_id": "a48ec002c76a6ce1538f85933902c3c6cf5c608f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1153999d09f2642f92378029c17e7207f86b76c6"
        },
        "date": 1696171840604,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54175178,
            "range": "± 447199",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 377744996,
            "range": "± 1373731",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28109197,
            "range": "± 38772",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "21c48959425c2ef6c8f349a7c85227617a217dc2",
          "message": "Bump docker/build-push-action from 4 to 5 (#771)\n\nBumps [docker/build-push-action](https://github.com/docker/build-push-action) from 4 to 5.\r\n- [Release notes](https://github.com/docker/build-push-action/releases)\r\n- [Commits](https://github.com/docker/build-push-action/compare/v4...v5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: docker/build-push-action\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-10-01T16:46:30+02:00",
          "tree_id": "9d3aa1315f2daa47be6c37a251759beb2855eb4c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/21c48959425c2ef6c8f349a7c85227617a217dc2"
        },
        "date": 1696171893384,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 57310267,
            "range": "± 552252",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 382289087,
            "range": "± 1306063",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29447475,
            "range": "± 280944",
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
          "id": "3c2ac126e60b1cfac06dccd36f022697dc06f743",
          "message": "Move dependabot to monthly intervals\n\nWe don't need all the spam",
          "timestamp": "2023-10-01T16:50:05+02:00",
          "tree_id": "126a16ef2fc8b90eb36145f622546bd50cf83751",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3c2ac126e60b1cfac06dccd36f022697dc06f743"
        },
        "date": 1696172025200,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 56906942,
            "range": "± 698552",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 379854257,
            "range": "± 1661764",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 29270302,
            "range": "± 213012",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "andy@andyfreeland.net",
            "name": "Andy Freeland",
            "username": "rouge8"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "32c433a29b17466d18ec5cef2b1a59af1389d7e6",
          "message": "Fix compatibility with release-gitter 2.2.0 (#808)",
          "timestamp": "2023-10-31T23:19:58Z",
          "tree_id": "d05f3535cbcf2a4ac424478557489a923ca8321c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/32c433a29b17466d18ec5cef2b1a59af1389d7e6"
        },
        "date": 1698794595346,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54746011,
            "range": "± 1107459",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369211934,
            "range": "± 1435734",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28782589,
            "range": "± 347408",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "50d90ee81e1867130e5887dfb68f96104f13b989",
          "message": "Bump @types/unzipper from 0.10.3 to 0.10.8 in /stylua-vscode (#816)\n\nBumps [@types/unzipper](https://github.com/DefinitelyTyped/DefinitelyTyped/tree/HEAD/types/unzipper) from 0.10.3 to 0.10.8.\r\n- [Release notes](https://github.com/DefinitelyTyped/DefinitelyTyped/releases)\r\n- [Commits](https://github.com/DefinitelyTyped/DefinitelyTyped/commits/HEAD/types/unzipper)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: \"@types/unzipper\"\r\n  dependency-type: direct:development\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-04T15:43:21+01:00",
          "tree_id": "ef35fd3595b857c2238f0e66d159e63591929de0",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/50d90ee81e1867130e5887dfb68f96104f13b989"
        },
        "date": 1699109197180,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 51412260,
            "range": "± 582273",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 366413919,
            "range": "± 1036558",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28043144,
            "range": "± 23660",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "30eb465b3815aba5a528f6235f112ef40ce419e4",
          "message": "Bump actions/setup-node from 3 to 4 (#814)\n\nBumps [actions/setup-node](https://github.com/actions/setup-node) from 3 to 4.\r\n- [Release notes](https://github.com/actions/setup-node/releases)\r\n- [Commits](https://github.com/actions/setup-node/compare/v3...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/setup-node\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-04T15:44:59+01:00",
          "tree_id": "fb41a19941691c6b2de784ecc2338ae10cd8533c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/30eb465b3815aba5a528f6235f112ef40ce419e4"
        },
        "date": 1699109227565,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37909258,
            "range": "± 218536",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 258048934,
            "range": "± 1643532",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 21027064,
            "range": "± 668836",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1f1d8f27887fe0d685376b2178d17444067f701f",
          "message": "Bump similar from 2.2.1 to 2.3.0 (#813)\n\nBumps [similar](https://github.com/mitsuhiko/similar) from 2.2.1 to 2.3.0.\r\n- [Changelog](https://github.com/mitsuhiko/similar/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/mitsuhiko/similar/compare/2.2.1...2.3.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: similar\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-04T15:45:15+01:00",
          "tree_id": "0c019f44e652c174645e197880b685b95882f7ed",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1f1d8f27887fe0d685376b2178d17444067f701f"
        },
        "date": 1699109252195,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37135788,
            "range": "± 492831",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 251733721,
            "range": "± 2540918",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20532670,
            "range": "± 697795",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "076b37e66b9a6bf7453a509c2c965756287428f2",
          "message": "Bump insta from 1.33.0 to 1.34.0 (#811)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.33.0 to 1.34.0.\r\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.33.0...1.34.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: insta\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-04T15:46:00+01:00",
          "tree_id": "4e6049473cb47aecf25bc63f069866669e1b2f80",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/076b37e66b9a6bf7453a509c2c965756287428f2"
        },
        "date": 1699109386165,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53795886,
            "range": "± 504561",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 367347539,
            "range": "± 7778034",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27934482,
            "range": "± 195809",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34089907+Barocena@users.noreply.github.com",
            "name": "Barış",
            "username": "Barocena"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5cf1ab3c754baa5482206d4eb165d19fc230b0f7",
          "message": "Fix SortRequireConfig on Wasm build (#818)\n\nFixed SortRequireConfig on Wasm build",
          "timestamp": "2023-11-05T13:56:32+01:00",
          "tree_id": "f6684ed2ca2c61b8f49556fc84b7deff4423966b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5cf1ab3c754baa5482206d4eb165d19fc230b0f7"
        },
        "date": 1699189123617,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 39156248,
            "range": "± 1123745",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260033447,
            "range": "± 2395814",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 21276414,
            "range": "± 1939933",
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
          "id": "8769d45e8825fa241e90d681cc04f53b685d8d77",
          "message": "Update vscode test runner (#819)",
          "timestamp": "2023-11-05T14:14:10+01:00",
          "tree_id": "d2de5e268351012d8b45bc3cf578d4d42965356a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8769d45e8825fa241e90d681cc04f53b685d8d77"
        },
        "date": 1699190242374,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54481873,
            "range": "± 849027",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 367559962,
            "range": "± 700306",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28038003,
            "range": "± 144553",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f6cab012b8dcf0025c318f232b4c6de0ee1eeaa",
          "message": "Bump axios from 0.27.2 to 1.6.0 in /stylua-npm-bin (#817)\n\nBumps [axios](https://github.com/axios/axios) from 0.27.2 to 1.6.0.\r\n- [Release notes](https://github.com/axios/axios/releases)\r\n- [Changelog](https://github.com/axios/axios/blob/v1.x/CHANGELOG.md)\r\n- [Commits](https://github.com/axios/axios/compare/v0.27.2...v1.6.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: axios\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-05T14:21:19+01:00",
          "tree_id": "fa06c2c533565e175cd57d521c5b1ca9846dbbc9",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5f6cab012b8dcf0025c318f232b4c6de0ee1eeaa"
        },
        "date": 1699190671761,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 53251773,
            "range": "± 5483126",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369860222,
            "range": "± 1686763",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28433216,
            "range": "± 81788",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "120d534229c6ed8c745fa9607fecd6ad6db6054d",
          "message": "Bump regex from 1.9.6 to 1.10.2 (#809)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.9.6 to 1.10.2.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.9.6...1.10.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-05T14:24:52+01:00",
          "tree_id": "18813e98d4c1d432bf86ba6e7251124707f2d771",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/120d534229c6ed8c745fa9607fecd6ad6db6054d"
        },
        "date": 1699190823141,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 41743684,
            "range": "± 418219",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261268613,
            "range": "± 2459481",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 21346264,
            "range": "± 254323",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6f2c342ba568e16d847169bf8ed2fbaa2d5a76e0",
          "message": "Bump serde_json from 1.0.107 to 1.0.108 (#812)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.107 to 1.0.108.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.107...v1.0.108)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-05T14:24:38+01:00",
          "tree_id": "47d187cee11b21eb51b7f542a08e4d4ac88f8996",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6f2c342ba568e16d847169bf8ed2fbaa2d5a76e0"
        },
        "date": 1699190875186,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 52478296,
            "range": "± 362829",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 366813777,
            "range": "± 474022",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 27749137,
            "range": "± 61764",
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
          "id": "d92ba7ed0c5c026a5dca96e8213de906ca3ca7ae",
          "message": "Disable dependabot\n\nThe noise is not useful right now",
          "timestamp": "2023-11-11T11:30:04+01:00",
          "tree_id": "e0cd85bf2eb2ea13de19d98ec533fc1a0f9f99b0",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d92ba7ed0c5c026a5dca96e8213de906ca3ca7ae"
        },
        "date": 1699698806797,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54100201,
            "range": "± 1357638",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 369751335,
            "range": "± 1079733",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28861703,
            "range": "± 333211",
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
          "id": "b5efedfb86b72a0e91015e5a4df7c78e75f9a47c",
          "message": "Actually disable dependabot",
          "timestamp": "2023-11-11T11:35:27+01:00",
          "tree_id": "4b9b390eeeb590a27b92dbf8ac5a4637b604bf29",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b5efedfb86b72a0e91015e5a4df7c78e75f9a47c"
        },
        "date": 1699699060006,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 38314886,
            "range": "± 657743",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 257517923,
            "range": "± 2433820",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 21122217,
            "range": "± 578975",
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
          "id": "951dba02d646877da07edb61437541635f46f75b",
          "message": "Fix cargo.lock",
          "timestamp": "2023-11-11T12:52:01+01:00",
          "tree_id": "1c9c6759667c20406f55c6d941b08a50857aa022",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/951dba02d646877da07edb61437541635f46f75b"
        },
        "date": 1699703718249,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 51776212,
            "range": "± 403756",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 368143233,
            "range": "± 636488",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28211938,
            "range": "± 146943",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3c8aee07530662abee6254e65d0d3b778b794104",
          "message": "Bump rustix from 0.38.13 to 0.38.21 (#820)\n\nBumps [rustix](https://github.com/bytecodealliance/rustix) from 0.38.13 to 0.38.21.\r\n- [Release notes](https://github.com/bytecodealliance/rustix/releases)\r\n- [Commits](https://github.com/bytecodealliance/rustix/compare/v0.38.13...v0.38.21)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: rustix\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-11-11T12:57:33+01:00",
          "tree_id": "e6a7819e4514d5d6578ab53296fdd31a89823475",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3c8aee07530662abee6254e65d0d3b778b794104"
        },
        "date": 1699703987104,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 39312441,
            "range": "± 859406",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 258366088,
            "range": "± 1527583",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 21008155,
            "range": "± 726651",
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
          "id": "d45a7744a2335a6e166a5094fe80c2263351a868",
          "message": "Fix large scale test tool",
          "timestamp": "2023-11-11T13:09:37+01:00",
          "tree_id": "61d68130e5574fb94d569be8f43209f9201b13b5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d45a7744a2335a6e166a5094fe80c2263351a868"
        },
        "date": 1699704780569,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 54595128,
            "range": "± 590421",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 370242410,
            "range": "± 619622",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 28587654,
            "range": "± 331855",
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
          "id": "711b0db673103ae5c96f89c8f5a29663c51077cd",
          "message": "Update to full-moon v0.19.0 (#821)\n\n* Update to full-moon v0.19.0\r\n\r\n* Fix handling of type assertion\r\n\r\n* Fix type assertion handling\r\n\r\n* Add test case\r\n\r\n* Fix compilation",
          "timestamp": "2023-11-11T14:34:37+01:00",
          "tree_id": "16e62051e73dc389cfb9c839bfa0ef59eea4f013",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/711b0db673103ae5c96f89c8f5a29663c51077cd"
        },
        "date": 1699709913734,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 61837115,
            "range": "± 511851",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 439232818,
            "range": "± 488988",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 32238752,
            "range": "± 247239",
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
          "id": "452e43bd62d89b71e73ea507c974923d16f30a73",
          "message": "Simplify access patterns for config\n\nAllow accessing properties directly\n\nFixes #805",
          "timestamp": "2023-11-11T14:46:26+01:00",
          "tree_id": "d53ae94e982213ad72bf55a940dc96fb6951cf1a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/452e43bd62d89b71e73ea507c974923d16f30a73"
        },
        "date": 1699710636197,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 66389874,
            "range": "± 3203833",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 496479143,
            "range": "± 19127093",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 34538836,
            "range": "± 1932880",
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
          "id": "095e2b0c79df0bac21915dd2deee9a0a25a9b382",
          "message": "Fix wasm output",
          "timestamp": "2023-11-11T14:55:54+01:00",
          "tree_id": "4c1006dcff69273c40c89bff00c8fe06fd3cb4d1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/095e2b0c79df0bac21915dd2deee9a0a25a9b382"
        },
        "date": 1699711107876,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37878422,
            "range": "± 582617",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 259241154,
            "range": "± 5480114",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20230866,
            "range": "± 590587",
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
          "id": "edf11fe9ba76905e80831ad0af96eb1d2967b2da",
          "message": "Keep multiline comments in place before commas (#822)\n\n* Keep multiline comments in place when formatting punctuated sequences\r\n\r\n* Update changelog",
          "timestamp": "2023-11-11T15:39:25+01:00",
          "tree_id": "0929c6cbd2b00adfa318e9d70d8704ff407a26a1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/edf11fe9ba76905e80831ad0af96eb1d2967b2da"
        },
        "date": 1699713816984,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 61258885,
            "range": "± 753525",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 452182249,
            "range": "± 3700932",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 31999127,
            "range": "± 304001",
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
          "id": "42a66fe16d8fdbf7041d7c011c217e0859407d7d",
          "message": "v0.19.0",
          "timestamp": "2023-11-12T12:51:30+01:00",
          "tree_id": "5a5749747de67543dc8a20452c27912cdf05461c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/42a66fe16d8fdbf7041d7c011c217e0859407d7d"
        },
        "date": 1699790041634,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37079419,
            "range": "± 825295",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 259913683,
            "range": "± 1493731",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20208024,
            "range": "± 153388",
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
          "id": "711e4a810086839b19f62728e9fd2f848b942e2b",
          "message": "Fix tests broken in debug mode\n\nWe remove some unrelated stmts from a test case, and gate another one behind a cfg flag",
          "timestamp": "2023-11-14T18:22:40+01:00",
          "tree_id": "5f56bd4a481c263f8e8e1e24114b67ee820d88af",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/711e4a810086839b19f62728e9fd2f848b942e2b"
        },
        "date": 1699982704288,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 38877813,
            "range": "± 511603",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 264928668,
            "range": "± 4298125",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20520921,
            "range": "± 1078230",
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
          "id": "df4d330ab873cb4f1798c4bf6a8ebc657e0a4b9a",
          "message": "v0.19.1",
          "timestamp": "2023-11-15T18:42:41+01:00",
          "tree_id": "721059026a92febceff0109e6f7b7e914a4378e4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/df4d330ab873cb4f1798c4bf6a8ebc657e0a4b9a"
        },
        "date": 1700070317550,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36601383,
            "range": "± 295092",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 263473267,
            "range": "± 2324654",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20417448,
            "range": "± 667510",
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
          "id": "4bd007e568380c9c2923de7ff545bcef424115b2",
          "message": "Fix handling of floor division when only luau flag enabled",
          "timestamp": "2023-11-18T18:48:41+01:00",
          "tree_id": "15625b25bb94cc479038ab775ef0662a6e50306e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4bd007e568380c9c2923de7ff545bcef424115b2"
        },
        "date": 1700329862494,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37847105,
            "range": "± 569530",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261762225,
            "range": "± 881014",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20443674,
            "range": "± 376879",
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
          "id": "712aa76d808111f548fbb77a9270d5742e88f1f7",
          "message": "Handle string interpolation table expression",
          "timestamp": "2023-11-19T12:20:09+01:00",
          "tree_id": "ea91e99a8724b5d20cdda1a520808f0d89a5d826",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/712aa76d808111f548fbb77a9270d5742e88f1f7"
        },
        "date": 1700392947014,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36530987,
            "range": "± 227371",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260424833,
            "range": "± 1369945",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20218697,
            "range": "± 183106",
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
          "id": "5de9a1da8a8dc628b98240e12ffe24c7fdbbea6e",
          "message": "Update external test cases (#825)\n\nCo-authored-by: JohnnyMorganz <JohnnyMorganz@users.noreply.github.com>",
          "timestamp": "2023-11-19T12:33:23+01:00",
          "tree_id": "081fba65f68ae37b7931bca2fa62beefe7a0332e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5de9a1da8a8dc628b98240e12ffe24c7fdbbea6e"
        },
        "date": 1700393735544,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36582634,
            "range": "± 236319",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260838180,
            "range": "± 1427020",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20329432,
            "range": "± 111414",
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
          "id": "a5ca6a43fde472d036ccb63f3c5ff246d29753af",
          "message": "Write files only if modified (#836)\n\n* Write files only if modified\r\n\r\n* Update changelog",
          "timestamp": "2023-12-23T12:38:20+01:00",
          "tree_id": "6a11212d96fad16d7019a41a7efbb38c44b988a4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a5ca6a43fde472d036ccb63f3c5ff246d29753af"
        },
        "date": 1703331637774,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37005642,
            "range": "± 839988",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 259990721,
            "range": "± 3058811",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20281198,
            "range": "± 404601",
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
          "id": "987c489503992a3b984d6b17840266219aad4cd9",
          "message": "Keep parentheses around compound type in table indexer (#837)\n\n* Add test case\r\n\r\n* Don't remove parens in table indexer\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2023-12-23T12:42:41+01:00",
          "tree_id": "579d5ca246c7340d7227de140045e3f40acd1e05",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/987c489503992a3b984d6b17840266219aad4cd9"
        },
        "date": 1703331902217,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36536384,
            "range": "± 1897979",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 264526685,
            "range": "± 1569624",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20799623,
            "range": "± 638769",
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
          "id": "ee290b02d1fa1b51a4006e20f13aabc35ea7fd5b",
          "message": "Don't put function definition parentheses on multiple lines when no parens (#838)\n\n* Add test case\r\n\r\n* Don't put on multiple lines\r\n\r\n* Update snapshots and changelog",
          "timestamp": "2023-12-23T12:56:43+01:00",
          "tree_id": "cdd562523899e64d35fa62a11226ef64eb2f9a02",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ee290b02d1fa1b51a4006e20f13aabc35ea7fd5b"
        },
        "date": 1703332732488,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36796961,
            "range": "± 410993",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261177036,
            "range": "± 4546565",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20434544,
            "range": "± 213753",
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
          "id": "15c22984902ceb3fb101f93b0be4544f8b36b416",
          "message": "Bump test versions in vscode\n\nCausing failures in CI",
          "timestamp": "2023-12-25T16:58:16+01:00",
          "tree_id": "932b4487195e5274fa5ed5a2b914920263a2cfa3",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/15c22984902ceb3fb101f93b0be4544f8b36b416"
        },
        "date": 1703520034288,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 39223615,
            "range": "± 352315",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 262822103,
            "range": "± 1074031",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20752386,
            "range": "± 764815",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "andrea.ros21@murena.io",
            "name": "andros21",
            "username": "andros21"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "94375edcd472e090362165f5b1cba398d178a5d5",
          "message": "Add new release artifact x86_64-unknown-linux-musl (#834)",
          "timestamp": "2023-12-30T13:00:10+01:00",
          "tree_id": "fc78fe1ade8be276ccd98e687f7200b6dbc6b32f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/94375edcd472e090362165f5b1cba398d178a5d5"
        },
        "date": 1703937742435,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36328365,
            "range": "± 206501",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261975484,
            "range": "± 1687642",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20254563,
            "range": "± 961533",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "33953936+dundargoc@users.noreply.github.com",
            "name": "dundargoc",
            "username": "dundargoc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1989671836ee291c3f2957222f1d768f22fea64d",
          "message": "Skip files in .gitignore. (#840)\n\nThis is a common behavior of formatters (e.g Ruff and Black) as it's\r\narguably a more useful default behavior.\r\n\r\nCloses https://github.com/JohnnyMorganz/StyLua/issues/833.",
          "timestamp": "2023-12-30T13:13:03+01:00",
          "tree_id": "eb240cfbec00ca3d796b3263409c5c0463f2402c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1989671836ee291c3f2957222f1d768f22fea64d"
        },
        "date": 1703938526892,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36685017,
            "range": "± 128562",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261835939,
            "range": "± 3171510",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20335696,
            "range": "± 604624",
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
          "id": "2e953c902143fb46a33a32f79d55b3dd68b9936b",
          "message": "Bump typescript and eslint versions in extension (#841)\n\nBump typescript and eslint version",
          "timestamp": "2023-12-30T13:13:15+01:00",
          "tree_id": "58cd57d7dfb2ff1f6cd3dcbcfd2210318dcea05e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2e953c902143fb46a33a32f79d55b3dd68b9936b"
        },
        "date": 1703938530126,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 39875776,
            "range": "± 1074578",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261161296,
            "range": "± 2129479",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20539385,
            "range": "± 157684",
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
          "id": "a61d98343f9110b255eb78579d0b0ffb5b068e48",
          "message": "Update changelog",
          "timestamp": "2023-12-30T13:17:39+01:00",
          "tree_id": "8a36141dc5d4482093c76e88594b158c24f6784d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a61d98343f9110b255eb78579d0b0ffb5b068e48"
        },
        "date": 1703938798962,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36396013,
            "range": "± 521447",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260411385,
            "range": "± 2841238",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20214859,
            "range": "± 994516",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "78505208+DervexHero@users.noreply.github.com",
            "name": "Dervex",
            "username": "DervexHero"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c3a9d977e5b9643c7a13fa45492f0ca454c287b0",
          "message": "Add status bar instead of error notifications (VSC extension) (#826)\n\n* Add option to hide formatting errors\r\n\r\n* Add status bar item and remove notification setting\r\n\r\n* Switch to language status item and update on editor change\r\n\r\n* Update changelog\r\n\r\n---------\r\n\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2023-12-30T14:02:53+01:00",
          "tree_id": "4a65c5635542e0b135723c3544710ccc49f63576",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c3a9d977e5b9643c7a13fa45492f0ca454c287b0"
        },
        "date": 1703941511941,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37128290,
            "range": "± 420140",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260470018,
            "range": "± 716202",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20222792,
            "range": "± 696772",
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
          "id": "76e611910ff217662f2516f5ad6516758acda345",
          "message": "extension: add `stylua.configPath` (#842)\n\nAdd `stylua.configPath`",
          "timestamp": "2023-12-30T14:19:07+01:00",
          "tree_id": "7c6361cf1e3efdb2190a656af2daef2b5fb4bf0a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/76e611910ff217662f2516f5ad6516758acda345"
        },
        "date": 1703942484766,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36252837,
            "range": "± 354997",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 263017081,
            "range": "± 1014752",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20369666,
            "range": "± 425400",
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
          "id": "af93051e6fff29c63fd89eb4b4105742f9bd3026",
          "message": "extension: add extension release workflow",
          "timestamp": "2023-12-30T14:19:45+01:00",
          "tree_id": "f8b870406c5250c063279a8fe339d59445cc69f4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/af93051e6fff29c63fd89eb4b4105742f9bd3026"
        },
        "date": 1703942524199,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36168802,
            "range": "± 988469",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260539517,
            "range": "± 1786639",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20496413,
            "range": "± 560539",
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
          "id": "6a05e27738be3eb5b3836c3d1a98952cccde8736",
          "message": "extension: add `stylua.verify` (#843)",
          "timestamp": "2023-12-30T14:27:05+01:00",
          "tree_id": "c0922cf30b4765faf906a8e9dcd16635a95d96b5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6a05e27738be3eb5b3836c3d1a98952cccde8736"
        },
        "date": 1703942965492,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37777104,
            "range": "± 368717",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261604521,
            "range": "± 557616",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20329495,
            "range": "± 476708",
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
          "id": "fcab9a958cb7cb6d4d2fb5782c7c02eb6467d991",
          "message": "Deprecate `stylua.releaseVersion`",
          "timestamp": "2023-12-30T15:04:06+01:00",
          "tree_id": "e9f1e531a850c672bb6ee5fbdde7ac25271ad647",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fcab9a958cb7cb6d4d2fb5782c7c02eb6467d991"
        },
        "date": 1703945181141,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36278651,
            "range": "± 89776",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 263400795,
            "range": "± 825471",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20221500,
            "range": "± 725594",
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
          "id": "2cf5e80e916cc692ee15cf10754d61c0a7d3d16f",
          "message": "extension: Show StyLua version in status bar item",
          "timestamp": "2023-12-30T15:34:05+01:00",
          "tree_id": "57c2ed31cd039d50e07332756edd59043ab5aa72",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2cf5e80e916cc692ee15cf10754d61c0a7d3d16f"
        },
        "date": 1703946982568,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 38101970,
            "range": "± 660031",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 262632787,
            "range": "± 3046999",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20830343,
            "range": "± 844256",
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
          "id": "863c0cc87187e7577427a904d209a31b392f34f4",
          "message": "Overhaul extension bundling (#844)\n\n* Overhaul extension bundling\r\n\r\n* Check version matches requested + stylua update\r\n\r\n* Prompt do not show again on updates\r\n\r\n* Add logging + check if binary found on PATH is executable\r\n\r\n* Update changelog",
          "timestamp": "2023-12-30T17:21:38+01:00",
          "tree_id": "cc312900d396315bbd30bb9a446875d23e29ba17",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/863c0cc87187e7577427a904d209a31b392f34f4"
        },
        "date": 1703953429820,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36618055,
            "range": "± 243717",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 260587946,
            "range": "± 1910833",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20300448,
            "range": "± 923604",
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
          "id": "f59ed91cf05cd6f4a5452d01d3c67882d5d65e53",
          "message": "extension: update README",
          "timestamp": "2023-12-30T17:35:35+01:00",
          "tree_id": "88665197e529e1ad65898e4dd401f3c0d5f24b86",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f59ed91cf05cd6f4a5452d01d3c67882d5d65e53"
        },
        "date": 1703954278786,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37868692,
            "range": "± 418511",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261672787,
            "range": "± 952152",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20741575,
            "range": "± 588927",
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
          "id": "8e611b4b61163c3c02cd76d892bd0bb77c0415b9",
          "message": "extension: v1.6.0",
          "timestamp": "2023-12-30T17:40:03+01:00",
          "tree_id": "d2c244d5d8e8204e0a79b3de0a24714f3bceff16",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8e611b4b61163c3c02cd76d892bd0bb77c0415b9"
        },
        "date": 1703954545882,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36530697,
            "range": "± 473025",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 263697134,
            "range": "± 2865396",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20178812,
            "range": "± 843359",
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
          "id": "a70daba2c91e0b372a7c52672fb272a873e10e85",
          "message": "extension: bump vscode engine version",
          "timestamp": "2023-12-30T17:45:44+01:00",
          "tree_id": "3cd9c0a3d03a17c22ff4dafd784a0b641d78dbcd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a70daba2c91e0b372a7c52672fb272a873e10e85"
        },
        "date": 1703954885655,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36012717,
            "range": "± 157336",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 264145886,
            "range": "± 960079",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20258820,
            "range": "± 485802",
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
          "id": "04d1551612d9a04d50f3e3b44eafbf81b6a292eb",
          "message": "extension: v1.6.1",
          "timestamp": "2023-12-30T18:23:47+01:00",
          "tree_id": "58533407346e105e088b29cb8cfc976bd6e7a87f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/04d1551612d9a04d50f3e3b44eafbf81b6a292eb"
        },
        "date": 1703957176603,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37147888,
            "range": "± 538021",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 261125744,
            "range": "± 2085335",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20600172,
            "range": "± 403650",
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
          "id": "fa568d50fce641670a7e5cadc48c83c5674bbb8a",
          "message": "extension: v1.6.2",
          "timestamp": "2023-12-31T02:17:23+01:00",
          "tree_id": "fd77792ecca9d5c4e057ad53660f78403e5b2ecd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fa568d50fce641670a7e5cadc48c83c5674bbb8a"
        },
        "date": 1703985600710,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37322477,
            "range": "± 1052656",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 263101043,
            "range": "± 2345567",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20400801,
            "range": "± 996615",
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
          "id": "6ff0856c5821d5409dd5e6c9a8ebf3a6cfc3a69f",
          "message": "extension: v1.6.3",
          "timestamp": "2024-01-06T18:23:26+01:00",
          "tree_id": "597655296af6b71687759820d53f08b4c1ca67d2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6ff0856c5821d5409dd5e6c9a8ebf3a6cfc3a69f"
        },
        "date": 1704561955863,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 40159321,
            "range": "± 587470",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 266657524,
            "range": "± 3483740",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20374273,
            "range": "± 197437",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bc3ce881eaaee46e8eb851366d33cba808d2a1f7",
          "message": "Bump follow-redirects from 1.15.1 to 1.15.4 in /stylua-npm-bin (#847)",
          "timestamp": "2024-01-12T20:02:44+01:00",
          "tree_id": "0898e4ccaccbcbdb35e12d1cd3c4b3f3f68a6750",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bc3ce881eaaee46e8eb851366d33cba808d2a1f7"
        },
        "date": 1705086293840,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35972881,
            "range": "± 533354",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 254976734,
            "range": "± 935564",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19988379,
            "range": "± 480770",
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
          "id": "bad95303787f6061d3a6854bb526bccdffe65cd8",
          "message": "Handle directory ignore paths and `--respect-ignores` (#852)",
          "timestamp": "2024-01-20T13:37:20+01:00",
          "tree_id": "e6def573e0f3d29e4368f50e7b7f092df8b68a15",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/bad95303787f6061d3a6854bb526bccdffe65cd8"
        },
        "date": 1705754374136,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35778608,
            "range": "± 1056989",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 255275157,
            "range": "± 1487210",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20181365,
            "range": "± 738742",
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
          "id": "c6e8619d2c0f679929c065a6101d99daca402cfd",
          "message": "v0.20.0",
          "timestamp": "2024-01-20T13:45:32+01:00",
          "tree_id": "5104e33a72c058a8a3f23d51a76a752075c5dd9f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c6e8619d2c0f679929c065a6101d99daca402cfd"
        },
        "date": 1705754868739,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35607921,
            "range": "± 393084",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 256425220,
            "range": "± 4798514",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20190534,
            "range": "± 852829",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "akari.ccino@gmail.com",
            "name": "magic-akari",
            "username": "magic-akari"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "049bb7c2c5ff0f764eec8ab3627e114ff23ddabf",
          "message": "feat(wasm): Support more targets (#848)",
          "timestamp": "2024-01-27T14:21:39+01:00",
          "tree_id": "91086176fb45905afe55c5b1683e48c7f574229c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/049bb7c2c5ff0f764eec8ab3627e114ff23ddabf"
        },
        "date": 1706361832313,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35370541,
            "range": "± 904167",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 253508519,
            "range": "± 566318",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20017030,
            "range": "± 668667",
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
          "id": "8642f6b279de1b3cdcc1a92e0156962839bd459a",
          "message": "Fix clippy lints",
          "timestamp": "2024-06-09T14:02:25+02:00",
          "tree_id": "56f62ebbad1a55f0e09dfe95fb2d1bdb3b445dad",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/8642f6b279de1b3cdcc1a92e0156962839bd459a"
        },
        "date": 1717934680175,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35651458,
            "range": "± 110148",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 259770742,
            "range": "± 780423",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20511142,
            "range": "± 295183",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "antoineauger@users.noreply.github.com",
            "name": "Antoine Auger",
            "username": "antoineauger"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1ad8d05863775c46a132e414878fbb910ac7d839",
          "message": "fix(stylua-npm-bin): adjust axios config to work with proxy env variables (#868)",
          "timestamp": "2024-06-19T21:20:09+02:00",
          "tree_id": "db9f1e08e6fb63474da0568c8739d96dd45b7d77",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1ad8d05863775c46a132e414878fbb910ac7d839"
        },
        "date": 1718824939138,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 39590421,
            "range": "± 618078",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 263172735,
            "range": "± 2202935",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 21566664,
            "range": "± 257067",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "caleb@alerque.com",
            "name": "Caleb Maclennan",
            "username": "alerque"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "88f8dd42a84ce74b44a3406cad74b0564311d0af",
          "message": "Apply clippy suggestions (#870)",
          "timestamp": "2024-06-28T22:03:50+02:00",
          "tree_id": "db9f1e08e6fb63474da0568c8739d96dd45b7d77",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/88f8dd42a84ce74b44a3406cad74b0564311d0af"
        },
        "date": 1719605149971,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36086217,
            "range": "± 692502",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 253297257,
            "range": "± 2673063",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19993028,
            "range": "± 803247",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "huajingyun@loongson.cn",
            "name": "huajingyun",
            "username": "huajingyun01"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "69183979f878da71b9a07bcd249e0176a740f28e",
          "message": "deps: bump libc from 0.2.148 to 0.2.155 (#862)",
          "timestamp": "2024-07-14T12:21:27+02:00",
          "tree_id": "e2e422f9af34d8e103752822f96a57ca6d9f36b1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/69183979f878da71b9a07bcd249e0176a740f28e"
        },
        "date": 1720952607171,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35135482,
            "range": "± 612067",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 249520589,
            "range": "± 588593",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19770996,
            "range": "± 391215",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "caleb@alerque.com",
            "name": "Caleb Maclennan",
            "username": "alerque"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7988cb749bedb425c978a10ad6a0a992e2982106",
          "message": "Add option for formatting with spaces between function names and arguments (#839)\n\n* Add option to define space after function definitions and calls\r\n\r\n* Add tests for new space after function name option\r\n\r\n* Implement formatting for new space after function name option\r\n\r\n* Rename option to `space_after_function_names`\r\n\r\n* Update changelog\r\n\r\n---------\r\n\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2024-10-26T13:11:57-05:00",
          "tree_id": "06f7070f3820d84978b05ebedadf87f5bf9e812e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7988cb749bedb425c978a10ad6a0a992e2982106"
        },
        "date": 1729966434254,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35390137,
            "range": "± 499473",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 269737232,
            "range": "± 4002186",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19366048,
            "range": "± 1022201",
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
          "id": "4d17163efa1fe00addaa1811354760ded8639309",
          "message": "Fix clippy lint issues",
          "timestamp": "2024-10-26T13:13:28-05:00",
          "tree_id": "c5f36f0e9b4ea58bf5e60a0333630de6f75a2975",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4d17163efa1fe00addaa1811354760ded8639309"
        },
        "date": 1729966542618,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 39425144,
            "range": "± 228116",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 286916263,
            "range": "± 1108413",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20310572,
            "range": "± 619251",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "013ddcfaa0f0241bfb110fe262f713d7969b51c3",
          "message": "Bump follow-redirects from 1.15.4 to 1.15.6 in /stylua-npm-bin (#856)\n\nBumps [follow-redirects](https://github.com/follow-redirects/follow-redirects) from 1.15.4 to 1.15.6.\r\n- [Release notes](https://github.com/follow-redirects/follow-redirects/releases)\r\n- [Commits](https://github.com/follow-redirects/follow-redirects/compare/v1.15.4...v1.15.6)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: follow-redirects\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-10-26T13:14:27-05:00",
          "tree_id": "07ccbac4a55eaba07943b2fff8907ce6ac19b98d",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/013ddcfaa0f0241bfb110fe262f713d7969b51c3"
        },
        "date": 1729966596632,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36270433,
            "range": "± 262090",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 277052312,
            "range": "± 705799",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19854036,
            "range": "± 744876",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f874a0b9c3ef6277d5b47585b19217a2fb60081e",
          "message": "Bump braces from 3.0.2 to 3.0.3 in /stylua-vscode (#900)\n\nBumps [braces](https://github.com/micromatch/braces) from 3.0.2 to 3.0.3.\r\n- [Changelog](https://github.com/micromatch/braces/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/micromatch/braces/compare/3.0.2...3.0.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: braces\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-10-26T13:30:16-05:00",
          "tree_id": "57814fd7ca16ed6de61472fc92236421aa76818c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f874a0b9c3ef6277d5b47585b19217a2fb60081e"
        },
        "date": 1729967545695,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36548408,
            "range": "± 479447",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 276224218,
            "range": "± 701122",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19930575,
            "range": "± 298604",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4b7a4f3b223182a19a909e97796325d9aa93d456",
          "message": "Bump webpack from 5.89.0 to 5.95.0 in /stylua-vscode (#901)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.89.0 to 5.95.0.\r\n- [Release notes](https://github.com/webpack/webpack/releases)\r\n- [Commits](https://github.com/webpack/webpack/compare/v5.89.0...v5.95.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: webpack\r\n  dependency-type: direct:development\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-10-26T13:30:28-05:00",
          "tree_id": "ee12480240352e1f4e4633447b93df129c99043c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4b7a4f3b223182a19a909e97796325d9aa93d456"
        },
        "date": 1729967551287,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 38152447,
            "range": "± 629908",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 277067286,
            "range": "± 2015752",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19951123,
            "range": "± 101656",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5737edef5e9fef05c327f9b2c799f4b66b8e7d9a",
          "message": "Bump axios from 1.6.0 to 1.7.4 in /stylua-npm-bin (#902)\n\nBumps [axios](https://github.com/axios/axios) from 1.6.0 to 1.7.4.\r\n- [Release notes](https://github.com/axios/axios/releases)\r\n- [Changelog](https://github.com/axios/axios/blob/v1.x/CHANGELOG.md)\r\n- [Commits](https://github.com/axios/axios/compare/v1.6.0...v1.7.4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: axios\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-10-26T13:30:38-05:00",
          "tree_id": "e9bd0f47db1fcf643049672c0e6e00cd28c26cac",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5737edef5e9fef05c327f9b2c799f4b66b8e7d9a"
        },
        "date": 1729967578211,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36498934,
            "range": "± 1111678",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 277780492,
            "range": "± 1504069",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19900799,
            "range": "± 372673",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "caleb@alerque.com",
            "name": "Caleb Maclennan",
            "username": "alerque"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9008ed2951f29a2fd5876cad2afeb7e58d4ec02b",
          "message": "Touch up symbol rename (#903)\n\nTouch up symbol rename missed in recent PR",
          "timestamp": "2024-10-26T14:20:25-05:00",
          "tree_id": "cb905fc19959f61abce19c3a74bb94d36a8009f2",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/9008ed2951f29a2fd5876cad2afeb7e58d4ec02b"
        },
        "date": 1729970553707,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36744458,
            "range": "± 448657",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 277587201,
            "range": "± 612831",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20002144,
            "range": "± 96723",
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
          "id": "4f94a4b9d907e11bdeede5e90fdb1ec01dce3533",
          "message": "Fix method call chain formatting with inline comments (#904)\n\nFix method call chain formatting with comments",
          "timestamp": "2024-10-26T14:30:34-05:00",
          "tree_id": "28e0995396772c022f1d5d78b48e531306d90efe",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/4f94a4b9d907e11bdeede5e90fdb1ec01dce3533"
        },
        "date": 1729971153185,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 35714164,
            "range": "± 1240011",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 279077819,
            "range": "± 1986666",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19867592,
            "range": "± 545892",
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
          "id": "f78c0630d2a6c23db213567fc361c065787fa703",
          "message": "Fix incorrect removal of semicolon before compound assignment causing ambiguous syntax error (#905)\n\n* Add test case\r\n\r\n* Handle compound assignment in requires semicolon check\r\n\r\n* Update changelog",
          "timestamp": "2024-10-26T14:56:22-05:00",
          "tree_id": "56c210aad614b9381a6a337af7d8023dceea65b5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f78c0630d2a6c23db213567fc361c065787fa703"
        },
        "date": 1729972709716,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37921758,
            "range": "± 839407",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 278418075,
            "range": "± 2212615",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20207024,
            "range": "± 988599",
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
          "id": "e4952b1a9d5fd3cece6b636e51840548e6f83222",
          "message": "Don't collapse compound type field if it contains comments (#906)\n\n* Add test case\r\n\r\n* Don't collapse compound type field if it contains types",
          "timestamp": "2024-10-26T15:12:15-05:00",
          "tree_id": "14da38591104482a70564fe23f172b1dc333a20a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/e4952b1a9d5fd3cece6b636e51840548e6f83222"
        },
        "date": 1729973693796,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 37100341,
            "range": "± 367778",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 276499997,
            "range": "± 1083574",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 20016911,
            "range": "± 308946",
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
          "id": "1cf56d147beb8c7f0665d4300343650fdbbaa2d9",
          "message": "Remove accidental print when formatting hanging returns (#907)",
          "timestamp": "2024-10-26T15:15:34-05:00",
          "tree_id": "fb587a7b79cb2fe695f1eb40f05ea5fa9747f5f5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1cf56d147beb8c7f0665d4300343650fdbbaa2d9"
        },
        "date": 1729973854901,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36873876,
            "range": "± 781895",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 275397070,
            "range": "± 788854",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19825660,
            "range": "± 245103",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "caleb@alerque.com",
            "name": "Caleb Maclennan",
            "username": "alerque"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cfc5d336992f35f222a9b891848d169afe8e574d",
          "message": "Update documentation of 'opinionated' to reflect project goals (#909)",
          "timestamp": "2024-11-16T15:52:38+01:00",
          "tree_id": "0c19e82ba033b4c026120af071020990b8d4fe02",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/cfc5d336992f35f222a9b891848d169afe8e574d"
        },
        "date": 1731768875921,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 36974819,
            "range": "± 408593",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 277054493,
            "range": "± 1162310",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 19973021,
            "range": "± 127705",
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
          "id": "daa6c6ed584a42cc092fac6516e85bd112f5b02a",
          "message": "Update to new full-moon version v1.1.1 (#854)\n\n* Update to new full-moon version\r\n\r\n* simplify err message\r\n\r\n* fix token\r\n\r\n* Support Lua version customisation\r\n\r\n* compile full moon with release optimisations\r\n\r\n* Update to Boxed anonymous function\r\n\r\n* Add proper handling for shebang\r\n\r\n* Set default lua version to all features, to make it easier to get started\r\n\r\nMeans its technically not a breaking change\r\n\r\n* Handle left associativity of type unions / intersections\r\n\r\n* Update callback hanging snapshot\r\n\r\nThis formatting is actually correct - we don't have type pack unions,\r\nso we should be hanging at the union type after the return\r\n\r\n* Support LuaJIT as a separate syntax option\r\n\r\n* Rename config option from `lua_version` to `syntax`\r\n\r\n* Fix tests and re-enable large example in debug mode\r\n\r\n* Use released full moon v1.1.0\r\n\r\n* Add command line option to configure syntax and update changelog\r\n\r\n* Bump full-moon to 1.1.1\r\n\r\n* Fix compilation for luau\r\n\r\n* Handle access modifier in type array and tables\r\n\r\n* Handle new type info and union set up\r\n\r\nSomewhat ugly code, designed to ensure there is snapshot diff.\r\nBut, the code can be made nicer in #910\r\n\r\n* Cleanup\r\n\r\n* Fix verify_ast for luajit\r\n\r\n* Fix wasm build on CI\r\n\r\n* Update changelog about luajit",
          "timestamp": "2024-11-16T20:22:19+01:00",
          "tree_id": "dfd5cfb5ac7b8ae12c77e1bae5c84a62b823e4f1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/daa6c6ed584a42cc092fac6516e85bd112f5b02a"
        },
        "date": 1731785050017,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 25899928,
            "range": "± 505734",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213656266,
            "range": "± 1251611",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14828791,
            "range": "± 158217",
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
          "id": "272490571da5a2e44ac594dcabbcad9e1344b635",
          "message": "Sync with latest full-moon test cases",
          "timestamp": "2024-11-16T20:29:03+01:00",
          "tree_id": "5e660fc48eece40fc6e64a55036903202c33ad47",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/272490571da5a2e44ac594dcabbcad9e1344b635"
        },
        "date": 1731785461727,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27020604,
            "range": "± 791163",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 217966502,
            "range": "± 17254632",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15195795,
            "range": "± 171140",
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
          "id": "c0db21d972d96ff031154e095b4d2e369a2dd6c2",
          "message": "Add another test case for hex overflow in ast verifier\n\nFixes #875",
          "timestamp": "2024-11-16T20:44:32+01:00",
          "tree_id": "8acb2883c8da3e40485ef3892a9d23ac996956bb",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/c0db21d972d96ff031154e095b4d2e369a2dd6c2"
        },
        "date": 1731786431498,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26177521,
            "range": "± 210560",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 212995089,
            "range": "± 708992",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14964262,
            "range": "± 197601",
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
          "id": "2c6a374fddf6588062bd3f287f15b2ba17fbba01",
          "message": "Remove legacy release artifacts from GitHub releases (#911)",
          "timestamp": "2024-11-16T21:10:58+01:00",
          "tree_id": "7d1105b4c300091c94885c16f7f1bee0c1ccae9a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/2c6a374fddf6588062bd3f287f15b2ba17fbba01"
        },
        "date": 1731787993342,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 25921744,
            "range": "± 233720",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213936652,
            "range": "± 1913752",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14882595,
            "range": "± 54274",
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
          "id": "5e4e8373bf7276b0adfde8b2557004035e2776c1",
          "message": "Don't ignore by default with `--stdin-filepath`, use `--respect-ignores` (#912)\n\nDon't ignore by default with `--stdin-filepath`",
          "timestamp": "2024-11-16T21:18:39+01:00",
          "tree_id": "de523df8ac7507283a70bb3b9782e25bfb76c53c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5e4e8373bf7276b0adfde8b2557004035e2776c1"
        },
        "date": 1731788434530,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26571856,
            "range": "± 446123",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213086262,
            "range": "± 503569",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14814339,
            "range": "± 107088",
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
          "id": "7caea0f29ed7415b2213df495f7f00e966a31373",
          "message": "Update VSCode extension to use stdin-filepath and respect-ignores (#913)",
          "timestamp": "2024-11-16T21:36:48+01:00",
          "tree_id": "99adab7ca746533bccef667fadc80cc8c95bbfb3",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7caea0f29ed7415b2213df495f7f00e966a31373"
        },
        "date": 1731789522810,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27692932,
            "range": "± 297139",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 216789198,
            "range": "± 1720279",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14985853,
            "range": "± 150105",
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
          "id": "1800e785131b970f2655fbd562cd83b4322fdd78",
          "message": "Update the versions of Luau supported in the readmes",
          "timestamp": "2024-11-16T21:44:48+01:00",
          "tree_id": "0200e1636e38f1bc7c4d475f3c64b0bbb071ca3e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1800e785131b970f2655fbd562cd83b4322fdd78"
        },
        "date": 1731790001809,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26211399,
            "range": "± 162462",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213877944,
            "range": "± 6069431",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14810457,
            "range": "± 90741",
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
          "id": "7225f3e605fd54c0cffbb939b7c3e2686a7f9219",
          "message": "extension: v1.7.0",
          "timestamp": "2024-11-16T21:49:15+01:00",
          "tree_id": "0f3b2fbb6621abc329ca76ba81edecf8c2a69e0a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7225f3e605fd54c0cffbb939b7c3e2686a7f9219"
        },
        "date": 1731790274292,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 25996056,
            "range": "± 147776",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 212948724,
            "range": "± 2104190",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14804959,
            "range": "± 66277",
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
          "id": "a7dae57dc2445d749cf1417ead3223899e2650e8",
          "message": "Fix link to Luau in readmes",
          "timestamp": "2024-11-16T22:29:18+01:00",
          "tree_id": "2b12d4241cad9111bc1e3753810ec2f8d90ce554",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a7dae57dc2445d749cf1417ead3223899e2650e8"
        },
        "date": 1731792672567,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 25934880,
            "range": "± 265600",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213410766,
            "range": "± 730526",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14880312,
            "range": "± 274774",
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
          "id": "b928a2ac4d1651568e78f2a7e0144d789f6326f0",
          "message": "extension: fix stdin-filepath for non-file documents",
          "timestamp": "2024-11-17T13:57:27+01:00",
          "tree_id": "1362f404d4344e2413951f60618826fa90e4401a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b928a2ac4d1651568e78f2a7e0144d789f6326f0"
        },
        "date": 1731848390109,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26097465,
            "range": "± 221318",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213198231,
            "range": "± 618760",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14880140,
            "range": "± 56433",
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
          "id": "0b83e89eb6b991af6ff359cadc9e1d2dcc12e8f9",
          "message": "Add CI testing for no default features and all features",
          "timestamp": "2024-11-17T14:05:42+01:00",
          "tree_id": "e4b2b1a5b964121e29a62d5d42b002d4a4d752de",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0b83e89eb6b991af6ff359cadc9e1d2dcc12e8f9"
        },
        "date": 1731848869638,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26022499,
            "range": "± 187834",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213454312,
            "range": "± 2320226",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14875044,
            "range": "± 191193",
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
          "id": "6c14f6f3a6c21a441805be5110df5be8b3da8c8d",
          "message": "Fix CI testing for all features",
          "timestamp": "2024-11-17T14:08:11+01:00",
          "tree_id": "c7564ff321d0aa16395666bd427302b2381356e4",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/6c14f6f3a6c21a441805be5110df5be8b3da8c8d"
        },
        "date": 1731849010966,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26323767,
            "range": "± 237982",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213569612,
            "range": "± 1624646",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14818178,
            "range": "± 100358",
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
          "id": "d7d532b4baf2bcf2adf1925d7248f659788e5b58",
          "message": "Update to full-moon 1.1.2 (#920)",
          "timestamp": "2024-11-17T16:38:15+01:00",
          "tree_id": "632420dc7d8539f3eeff3d7fcc6c0ec0cca1e357",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d7d532b4baf2bcf2adf1925d7248f659788e5b58"
        },
        "date": 1731858007940,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26349129,
            "range": "± 229413",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213002284,
            "range": "± 530655",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15119212,
            "range": "± 318228",
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
          "id": "de63b6621d95b77288e7bae0abacc3637f206cb6",
          "message": "Run CorePackages test with Luau syntax",
          "timestamp": "2024-11-17T17:03:50+01:00",
          "tree_id": "7cb54e1632dd2dae4b36074fbf6edd897e03abec",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/de63b6621d95b77288e7bae0abacc3637f206cb6"
        },
        "date": 1731859545150,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26188630,
            "range": "± 360633",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 212844910,
            "range": "± 2105929",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14945789,
            "range": "± 92121",
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
          "id": "597268389a7d486f6856a4525dbd11678e324cff",
          "message": "Improve error reporting on full moon errors (#921)\n\n* Improve error reporting on full moon errors\r\n\r\n* Add space before error\r\n\r\n* Improve printing for verification AST error\r\n\r\n* Remove newline",
          "timestamp": "2024-11-17T17:13:26+01:00",
          "tree_id": "8950eb4bc05fcf60e5f564065ff12fd313350dfa",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/597268389a7d486f6856a4525dbd11678e324cff"
        },
        "date": 1731860129481,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27715887,
            "range": "± 788472",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 216027915,
            "range": "± 1734357",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15222187,
            "range": "± 311069",
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
          "id": "d11797b39099799c65f219edf6a45492baefe5f3",
          "message": "Switch to closest file configuration resolution (#916)\n\n* Closest file config resolution\r\n\r\n* Respect config path override for stdin filepath",
          "timestamp": "2024-11-17T17:13:52+01:00",
          "tree_id": "a9fe917f34c6492feb2ee08a4911e5316c6d6a0a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d11797b39099799c65f219edf6a45492baefe5f3"
        },
        "date": 1731860142709,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26076041,
            "range": "± 521866",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213567010,
            "range": "± 1280968",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14827650,
            "range": "± 43017",
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
          "id": "26047670e05ba310afe9c1c1f91be6749e1f3ac9",
          "message": "Remove deprecated access patterns on `Config` struct (#922)\n\nRemove deprecated access patterns",
          "timestamp": "2024-11-17T17:23:11+01:00",
          "tree_id": "c29706a47ff1b630168ce44fbe21d9a539b68328",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/26047670e05ba310afe9c1c1f91be6749e1f3ac9"
        },
        "date": 1731860707020,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 28081771,
            "range": "± 525050",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215678611,
            "range": "± 762848",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15221293,
            "range": "± 252833",
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
          "id": "d8f820e14dd230492533b5037cde0bb920e492b4",
          "message": "collapse_simple_stmt: check if return expressions are \"simple\" (#923)\n\n* Add test case\r\n\r\n* Check if return expression is simple under collapse_simple_stmt\r\n\r\n* Update changelog and snapshots\r\n\r\n* Update changelog link",
          "timestamp": "2024-11-17T17:47:44+01:00",
          "tree_id": "9e1ccac426d1c0f48353f1041562bd5917ae1322",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d8f820e14dd230492533b5037cde0bb920e492b4"
        },
        "date": 1731862181555,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26078292,
            "range": "± 132090",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213122398,
            "range": "± 391570",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14870512,
            "range": "± 44687",
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
          "id": "ef4ae379561c665c20e74e75d45a1a6e0d57b994",
          "message": "Update contents of readme about syntax selection (#914)",
          "timestamp": "2024-11-17T17:49:41+01:00",
          "tree_id": "7ba5750e1e7a9294557b0c8cc3ad03cc08953ca6",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/ef4ae379561c665c20e74e75d45a1a6e0d57b994"
        },
        "date": 1731862289468,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26012889,
            "range": "± 265143",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 212841359,
            "range": "± 873312",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14772271,
            "range": "± 339784",
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
          "id": "08e536f3a02d9a07b0991726897e0013ff78dd21",
          "message": "v2.0.0",
          "timestamp": "2024-11-17T17:54:04+01:00",
          "tree_id": "cd8cca8704d6d0b083983820affe73edac3f2570",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/08e536f3a02d9a07b0991726897e0013ff78dd21"
        },
        "date": 1731862601385,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26078463,
            "range": "± 271685",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215061102,
            "range": "± 513930",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14884680,
            "range": "± 144584",
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
          "id": "51da190c037cbd25a50cc09c26410f2f6304cada",
          "message": "Bump rust version in Dockerfile",
          "timestamp": "2024-11-17T22:02:09+01:00",
          "tree_id": "d405aadb07a643daaafb1e2344bb0be870e215fe",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/51da190c037cbd25a50cc09c26410f2f6304cada"
        },
        "date": 1731877512597,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 28088873,
            "range": "± 1013656",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 217248766,
            "range": "± 2217752",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15106655,
            "range": "± 109244",
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
          "id": "a8207130bc40dce73c8258b5d0dbd6b6e8a9fb64",
          "message": "Fix CLI overrides not applying on top of resolved configuration (#926)\n\n* Apply CLI overrides when configuration is found\r\n\r\n* Update changelog",
          "timestamp": "2024-11-17T22:06:03+01:00",
          "tree_id": "3ba7df131085e788dafe7e7149d2e01500dbb47e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a8207130bc40dce73c8258b5d0dbd6b6e8a9fb64"
        },
        "date": 1731877675693,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26462108,
            "range": "± 507194",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 214302340,
            "range": "± 3534208",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14802498,
            "range": "± 139706",
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
          "id": "746d832830c726daa1309263f8fb0ab33c05dcb6",
          "message": "Update external test cases (#927)",
          "timestamp": "2024-11-18T18:42:45+01:00",
          "tree_id": "f5b5f01e795b7c2d629785216a189ffc6ad820bb",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/746d832830c726daa1309263f8fb0ab33c05dcb6"
        },
        "date": 1731951877831,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26214141,
            "range": "± 210741",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 214141838,
            "range": "± 2204547",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14862050,
            "range": "± 50481",
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
          "id": "60498492cac3860093ed31f4be74d73893ab0460",
          "message": "Update readme about output formats",
          "timestamp": "2024-11-18T19:21:26+01:00",
          "tree_id": "64dbb98f8835bdb81b7e9841c649e0b54adf668e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/60498492cac3860093ed31f4be74d73893ab0460"
        },
        "date": 1731954394562,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27308025,
            "range": "± 363725",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215405653,
            "range": "± 2637795",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14985325,
            "range": "± 322188",
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
          "id": "d6b8e7e57fdd3bd4d87df1c6859e4df327b5ee0c",
          "message": "Log if an ignore file was resolved during --respect-ignores",
          "timestamp": "2024-11-18T19:41:51+01:00",
          "tree_id": "df6fe0de9f5be33c49db67f3ac37133842ca592c",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d6b8e7e57fdd3bd4d87df1c6859e4df327b5ee0c"
        },
        "date": 1731955433966,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27256342,
            "range": "± 421877",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 214650791,
            "range": "± 396739",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14940907,
            "range": "± 100799",
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
          "id": "1daf4c18fb6baf687cc41082f1cc6905ced226a4",
          "message": "v2.0.1",
          "timestamp": "2024-11-18T19:48:54+01:00",
          "tree_id": "61ccd76143d77aa6f585054d8f49038cf80d078a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1daf4c18fb6baf687cc41082f1cc6905ced226a4"
        },
        "date": 1731955861041,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27283114,
            "range": "± 585906",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 214858865,
            "range": "± 1634560",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14956917,
            "range": "± 64011",
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
          "id": "d67c77b6f73ccab63d544adffe9cf3d859e71fa4",
          "message": "extension: fix debug message",
          "timestamp": "2024-11-30T13:08:46+01:00",
          "tree_id": "748e1d528d35cd374ba4e6d22ff8ce0888df0bc5",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d67c77b6f73ccab63d544adffe9cf3d859e71fa4"
        },
        "date": 1732968669235,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26707366,
            "range": "± 302034",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215446517,
            "range": "± 5485449",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14926605,
            "range": "± 729254",
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
          "id": "f581279895a7040a36abaea4a6c8a6f50f112cd7",
          "message": "Use configuration from cwd when formatting from stdin (#931)\n\nUse configuration from current working directory when formatting\r\nfrom stdin and no stdin filepath provided",
          "timestamp": "2024-11-30T13:36:12+01:00",
          "tree_id": "2b38d42e749193fb6ca9cc2e4015ebba2993a022",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/f581279895a7040a36abaea4a6c8a6f50f112cd7"
        },
        "date": 1732970290752,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27115172,
            "range": "± 327585",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 213853983,
            "range": "± 1558803",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15052608,
            "range": "± 698086",
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
          "id": "d570166502f922e96ace770be7f773e72b185d5b",
          "message": "Luau: fix indentation of leading token in union/intersection when hanging (#933)\n\n* Add test case\r\n\r\n* Fix indentation formatting of leading token in hanging type info\r\n\r\n* Update changelog and snapshots",
          "timestamp": "2024-11-30T13:49:46+01:00",
          "tree_id": "7491237f9ed911237e574b5d9eafb7eb6ecad464",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/d570166502f922e96ace770be7f773e72b185d5b"
        },
        "date": 1732971127116,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26440805,
            "range": "± 232234",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 216605772,
            "range": "± 1767826",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15130020,
            "range": "± 749219",
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
          "id": "59be1c313b50d3ee72f988c840f9a178792a4c72",
          "message": "v2.0.2",
          "timestamp": "2024-12-07T14:57:04+01:00",
          "tree_id": "de153ee9a4ad4c4aa2e86f28df050ee0b0908f16",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/59be1c313b50d3ee72f988c840f9a178792a4c72"
        },
        "date": 1733579955796,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26702839,
            "range": "± 503402",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215382709,
            "range": "± 2104020",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14917308,
            "range": "± 39689",
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
          "id": "1705207fa553ef4b0545911143c22ac0ec021951",
          "message": "Don't remove parentheses in `(expr :: assertion) < expr` (#941)\n\n* Add test case\r\n\r\n* Don't remove parentheses in type assertion\r\n\r\n* Update snapshots\r\n\r\n* Update changelog",
          "timestamp": "2024-12-24T14:46:23Z",
          "tree_id": "c5ab68e24c07fcf8217c9c1906009b0d401601be",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1705207fa553ef4b0545911143c22ac0ec021951"
        },
        "date": 1735051697123,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26929642,
            "range": "± 339225",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 216943690,
            "range": "± 621424",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14790065,
            "range": "± 72646",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ukendio@gmail.com",
            "name": "Marcus",
            "username": "Ukendio"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b349c0aa0eb036bb49d4ce82eaabbb173f627a6f",
          "message": "Bump fullmoon to 1.2.0 (#945)\n\n* Bump fullmoon to 1.2.0\r\n\r\nThis is the latest version that supports user defined type functions.\r\n\r\n* Update lock file\r\n\r\n* Revert changes to lockfile\r\n\r\n* Only repin full moon in lock file\r\n\r\n---------\r\n\r\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2025-01-12T12:27:29Z",
          "tree_id": "6d5d22ed6b7c581b04d6e10c00c74e7bbfa5fb66",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/b349c0aa0eb036bb49d4ce82eaabbb173f627a6f"
        },
        "date": 1736684967803,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26803673,
            "range": "± 196067",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215826233,
            "range": "± 456001",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14838888,
            "range": "± 204662",
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
          "id": "57408a808b234b3c8a3bd69dcfc8c21b734e695a",
          "message": "Luau: Support user-defined type functions (#947)\n\n* Support user-defined type functions\r\n\r\n* Add test case\r\n\r\n* Update changelog",
          "timestamp": "2025-01-12T13:37:32Z",
          "tree_id": "3c69636c18b1353c9b1836ebcff380b39652d91a",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/57408a808b234b3c8a3bd69dcfc8c21b734e695a"
        },
        "date": 1736689169886,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26391382,
            "range": "± 179566",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 216889809,
            "range": "± 1469012",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14849246,
            "range": "± 242011",
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
          "id": "a371dea3e293b338a477d43a8c17cc27c7f6412a",
          "message": "Update external test cases (#948)\n\nCo-authored-by: JohnnyMorganz <JohnnyMorganz@users.noreply.github.com>",
          "timestamp": "2025-01-13T17:29:20Z",
          "tree_id": "aa395ad1ea56c25059cfb28a501a4b018f655beb",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/a371dea3e293b338a477d43a8c17cc27c7f6412a"
        },
        "date": 1736789472706,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26519710,
            "range": "± 148781",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 216229752,
            "range": "± 834546",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14876870,
            "range": "± 84208",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "4740959+eitamal@users.noreply.github.com",
            "name": "Itamar Lencovsky",
            "username": "eitamal"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7c07c54f9cfc9814fa27792811191ab886be5bbb",
          "message": "fix: use \"summary\" for the missing `--check` error message (#949)",
          "timestamp": "2025-01-13T18:35:30+01:00",
          "tree_id": "4b1c492a3bda235bb5befa92307c487c000e7d55",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/7c07c54f9cfc9814fa27792811191ab886be5bbb"
        },
        "date": 1736789849936,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 28081999,
            "range": "± 662032",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 217963204,
            "range": "± 516365",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14863158,
            "range": "± 162372",
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
          "id": "3d4b42498194e1849c20d0d5e24a470d224f8159",
          "message": "Fix clippy lints (#973)",
          "timestamp": "2025-04-21T14:54:50+02:00",
          "tree_id": "5bec99e4b3673aa374a34ff5fac56dfa76583752",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/3d4b42498194e1849c20d0d5e24a470d224f8159"
        },
        "date": 1745240198866,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27231814,
            "range": "± 633691",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 217167480,
            "range": "± 1376944",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14791974,
            "range": "± 238409",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "petertripp@gmail.com",
            "name": "Peter Tripp",
            "username": "notpeter"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "712544922b6b62c2333a54ee9b733d253ecb8ba0",
          "message": "Document --stdin-filepath in README.md (#954)\n\n* Document --stdin-filepath in README.md. Fixes #953\n\n* Update README.md\n\n---------\n\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2025-04-21T14:58:53+02:00",
          "tree_id": "da68e8d99b12bb4c2a94564c276a72dd245b266f",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/712544922b6b62c2333a54ee9b733d253ecb8ba0"
        },
        "date": 1745240439401,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26310488,
            "range": "± 99515",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 215379233,
            "range": "± 685268",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14731377,
            "range": "± 31058",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "16a503d06bd6d2d135dcead9cd74068edefb52b4",
          "message": "Bump serialize-javascript and mocha in /stylua-vscode (#959)\n\nBumps [serialize-javascript](https://github.com/yahoo/serialize-javascript) to 6.0.2 and updates ancestor dependency [mocha](https://github.com/mochajs/mocha). These dependencies need to be updated together.\n\n\nUpdates `serialize-javascript` from 6.0.1 to 6.0.2\n- [Release notes](https://github.com/yahoo/serialize-javascript/releases)\n- [Commits](https://github.com/yahoo/serialize-javascript/compare/v6.0.1...v6.0.2)\n\nUpdates `mocha` from 10.2.0 to 10.8.2\n- [Release notes](https://github.com/mochajs/mocha/releases)\n- [Changelog](https://github.com/mochajs/mocha/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/mochajs/mocha/compare/v10.2.0...v10.8.2)\n\n---\nupdated-dependencies:\n- dependency-name: serialize-javascript\n  dependency-type: indirect\n- dependency-name: mocha\n  dependency-type: direct:development\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-04-21T14:59:31+02:00",
          "tree_id": "b9127d16daed2753dc5134fd236de21acc7219cf",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/16a503d06bd6d2d135dcead9cd74068edefb52b4"
        },
        "date": 1745240482756,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26610100,
            "range": "± 203561",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 214174208,
            "range": "± 2652305",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14629587,
            "range": "± 337589",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "aad1b7de1d1f8c19055060231b626543bc4444a5",
          "message": "Bump crossbeam-channel from 0.5.11 to 0.5.15 (#968)\n\nBumps [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam) from 0.5.11 to 0.5.15.\n- [Release notes](https://github.com/crossbeam-rs/crossbeam/releases)\n- [Changelog](https://github.com/crossbeam-rs/crossbeam/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/crossbeam-rs/crossbeam/compare/crossbeam-channel-0.5.11...crossbeam-channel-0.5.15)\n\n---\nupdated-dependencies:\n- dependency-name: crossbeam-channel\n  dependency-version: 0.5.15\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-04-21T15:00:22+02:00",
          "tree_id": "e2d96f8b2d7b5247ae0fbf8cff3e0e9a23c96520",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/aad1b7de1d1f8c19055060231b626543bc4444a5"
        },
        "date": 1745240548890,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27049277,
            "range": "± 335008",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 219096579,
            "range": "± 2481107",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 14749034,
            "range": "± 75184",
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
          "id": "5138d48e741a7ea4a8379ef2b1ac45d03ab332e5",
          "message": "Bump full_moon to v2.0.0 (#974)\n\n* Bump full_moon to v2.0.0\n\n* Fix compound location switch",
          "timestamp": "2025-04-21T15:18:54+02:00",
          "tree_id": "fbbb5377d85eefccc460dfa7336e51b3dd68de98",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/5138d48e741a7ea4a8379ef2b1ac45d03ab332e5"
        },
        "date": 1745241650051,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27812010,
            "range": "± 449742",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 229822854,
            "range": "± 826127",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15344626,
            "range": "± 385076",
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
          "id": "36981d7876d1ad5883de1c7febe365130ab98dd7",
          "message": "Support Luau attributes (#975)\n\nSupport luau attributes",
          "timestamp": "2025-04-21T15:40:22+02:00",
          "tree_id": "f0a8e451f16800963a1a2c64438a043e1df0150e",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/36981d7876d1ad5883de1c7febe365130ab98dd7"
        },
        "date": 1745242944883,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27067181,
            "range": "± 247431",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 230647706,
            "range": "± 1545710",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15304308,
            "range": "± 194861",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "91544758+phanen@users.noreply.github.com",
            "name": "phanium",
            "username": "phanen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "923e912f18cf9380d98d6fac39b74a2201a49e59",
          "message": "Fix panic when `--respect-ignores --stdin-filepath` on external path to cwd (#969)\n\n* Fix panic when `--respect-ignores --stdin-filepath` on external path to cwd\n\n* Update changelog\n\n---------\n\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2025-04-21T15:43:22+02:00",
          "tree_id": "b526bf4c40132e3f9133c2949ad1bf6bf668985b",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/923e912f18cf9380d98d6fac39b74a2201a49e59"
        },
        "date": 1745243110873,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26595247,
            "range": "± 1122626",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 227725516,
            "range": "± 1065472",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15234533,
            "range": "± 394492",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0aa2b48ac875071af7b69d24ff805a5645581991",
          "message": "Bump axios from 1.7.4 to 1.8.2 in /stylua-npm-bin (#976)\n\nBumps [axios](https://github.com/axios/axios) from 1.7.4 to 1.8.2.\n- [Release notes](https://github.com/axios/axios/releases)\n- [Changelog](https://github.com/axios/axios/blob/v1.x/CHANGELOG.md)\n- [Commits](https://github.com/axios/axios/compare/v1.7.4...v1.8.2)\n\n---\nupdated-dependencies:\n- dependency-name: axios\n  dependency-version: 1.8.2\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-04-21T15:58:58+02:00",
          "tree_id": "c3e7fda47438319d93cc5e705bb92b8478c1f211",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/0aa2b48ac875071af7b69d24ff805a5645581991"
        },
        "date": 1745244054961,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 27776377,
            "range": "± 395639",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 229924243,
            "range": "± 1316956",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15245034,
            "range": "± 145040",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "nilscosmo@hotmail.com",
            "name": "Nils Gerersdorfer",
            "username": "Kuuzoo"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fb9119b1041428c6337439e12d848d6a4654f7db",
          "message": "Support Cfx Lua Syntax (#972)\n\n* start with cfxlua support\n\n* fix: enhance CFXLua support for backtick string literals in format_token function\n\n* CFXLua -> CfxLua\n\n* Add tests\n\n* Add CfxLua to CI\n\n* Move compound op formatting to generic location\n\n* Handle set constructor field\n\n* Extract out string literal handling\n\n* Ensure cfxlua is enabled in \"All\" mode, and fix symbol parsing\n\n* Update snapshots\n\n* Handle in unpacking\n\n* Update snapshot\n\n* Formatting\n\n* Update changelog and readme\n\n* Fix compilation\n\n* Enable compound op for cfxlua\n\n---------\n\nCo-authored-by: JohnnyMorganz <johnnymorganz@outlook.com>",
          "timestamp": "2025-04-21T17:04:34+02:00",
          "tree_id": "5641c3233fa67f84dcaae6601611ccedcfefc9cd",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/fb9119b1041428c6337439e12d848d6a4654f7db"
        },
        "date": 1745247994889,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26394700,
            "range": "± 191596",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 228553467,
            "range": "± 542814",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15195844,
            "range": "± 63534",
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
          "id": "1b49ab037fb021b1bb58886446b422944564e708",
          "message": "Update GitHub Action runs (#977)\n\n* commit base\n\n* Bump to 22.04\n\n* Use native arm runners for linux arm release\n\n* Create a linux aarch64 musl target\n\n* Update upload artifact version\n\n* Update main workflow\n\n* Delete dummy workflow\n\n* Update changelog\n\n* Publish ARM docker images",
          "timestamp": "2025-04-21T17:31:16+02:00",
          "tree_id": "bdb644fe1d1d901eae5f5b8e594d2fc2addcf5b1",
          "url": "https://github.com/JohnnyMorganz/StyLua/commit/1b49ab037fb021b1bb58886446b422944564e708"
        },
        "date": 1745249587066,
        "tool": "cargo",
        "benches": [
          {
            "name": "format date.lua",
            "value": 26958902,
            "range": "± 302908",
            "unit": "ns/iter"
          },
          {
            "name": "format docgen.lua",
            "value": 229068220,
            "range": "± 1228364",
            "unit": "ns/iter"
          },
          {
            "name": "format nested_tables.lua",
            "value": 15207956,
            "range": "± 370416",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}