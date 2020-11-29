# How to do the testing of a new ruleset

## Get data from your robot to mock inputs
./do_record.sh <filename>

, where you probably want to make your filename `testfiles/<name of your choice>_mock.yaml`. Otherwise you'll have to rename it to that, because the path and name are important!

## Create your setup file (or copy one)
Make sure the file is called `testfiles/<same name of your choice>_setup.yaml`, and you configure it properly like:
```
start_actionset_file: testfiles/<same name of your choise>_cfg.yaml
```

## Create your ruleset/config file (or copy one)
Make sure the file is called `testfiles/<same name of your choice>_cfg.yaml`.

## Create your expectations file (or copy one)
Make sure the file is called `testfiles/<same name of your choice>_cfg.yaml`.

## Add another test function to src/brain_test.rs
Again, copy and adapt examples on that file, they are straightforward.

## TEST!
cargo test --
