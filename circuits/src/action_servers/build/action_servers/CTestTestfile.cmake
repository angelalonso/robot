# CMake generated Testfile for 
# Source directory: /home/aaf/ros2_ws/circuits/src/action_servers
# Build directory: /home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(cppcheck "/usr/bin/python3.10" "-u" "/opt/ros/rolling/share/ament_cmake_test/cmake/run_test.py" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/cppcheck.xunit.xml" "--package-name" "action_servers" "--output-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/ament_cppcheck/cppcheck.txt" "--command" "/opt/ros/rolling/bin/ament_cppcheck" "--xunit-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/cppcheck.xunit.xml" "--include_dirs" "/home/aaf/ros2_ws/circuits/src/action_servers/include")
set_tests_properties(cppcheck PROPERTIES  LABELS "cppcheck;linter" TIMEOUT "300" WORKING_DIRECTORY "/home/aaf/ros2_ws/circuits/src/action_servers" _BACKTRACE_TRIPLES "/opt/ros/rolling/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/rolling/share/ament_cmake_cppcheck/cmake/ament_cppcheck.cmake;66;ament_add_test;/opt/ros/rolling/share/ament_cmake_cppcheck/cmake/ament_cmake_cppcheck_lint_hook.cmake;87;ament_cppcheck;/opt/ros/rolling/share/ament_cmake_cppcheck/cmake/ament_cmake_cppcheck_lint_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;21;ament_execute_extensions;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_package.cmake;66;ament_execute_extensions;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;91;ament_package;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;0;")
add_test(flake8 "/usr/bin/python3.10" "-u" "/opt/ros/rolling/share/ament_cmake_test/cmake/run_test.py" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/flake8.xunit.xml" "--package-name" "action_servers" "--output-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/ament_flake8/flake8.txt" "--command" "/opt/ros/rolling/bin/ament_flake8" "--xunit-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/flake8.xunit.xml")
set_tests_properties(flake8 PROPERTIES  LABELS "flake8;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/aaf/ros2_ws/circuits/src/action_servers" _BACKTRACE_TRIPLES "/opt/ros/rolling/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/rolling/share/ament_cmake_flake8/cmake/ament_flake8.cmake;63;ament_add_test;/opt/ros/rolling/share/ament_cmake_flake8/cmake/ament_cmake_flake8_lint_hook.cmake;18;ament_flake8;/opt/ros/rolling/share/ament_cmake_flake8/cmake/ament_cmake_flake8_lint_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;21;ament_execute_extensions;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_package.cmake;66;ament_execute_extensions;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;91;ament_package;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;0;")
add_test(lint_cmake "/usr/bin/python3.10" "-u" "/opt/ros/rolling/share/ament_cmake_test/cmake/run_test.py" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/lint_cmake.xunit.xml" "--package-name" "action_servers" "--output-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/ament_lint_cmake/lint_cmake.txt" "--command" "/opt/ros/rolling/bin/ament_lint_cmake" "--xunit-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/lint_cmake.xunit.xml")
set_tests_properties(lint_cmake PROPERTIES  LABELS "lint_cmake;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/aaf/ros2_ws/circuits/src/action_servers" _BACKTRACE_TRIPLES "/opt/ros/rolling/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/rolling/share/ament_cmake_lint_cmake/cmake/ament_lint_cmake.cmake;47;ament_add_test;/opt/ros/rolling/share/ament_cmake_lint_cmake/cmake/ament_cmake_lint_cmake_lint_hook.cmake;21;ament_lint_cmake;/opt/ros/rolling/share/ament_cmake_lint_cmake/cmake/ament_cmake_lint_cmake_lint_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;21;ament_execute_extensions;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_package.cmake;66;ament_execute_extensions;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;91;ament_package;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;0;")
add_test(pep257 "/usr/bin/python3.10" "-u" "/opt/ros/rolling/share/ament_cmake_test/cmake/run_test.py" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/pep257.xunit.xml" "--package-name" "action_servers" "--output-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/ament_pep257/pep257.txt" "--command" "/opt/ros/rolling/bin/ament_pep257" "--xunit-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/pep257.xunit.xml")
set_tests_properties(pep257 PROPERTIES  LABELS "pep257;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/aaf/ros2_ws/circuits/src/action_servers" _BACKTRACE_TRIPLES "/opt/ros/rolling/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/rolling/share/ament_cmake_pep257/cmake/ament_pep257.cmake;41;ament_add_test;/opt/ros/rolling/share/ament_cmake_pep257/cmake/ament_cmake_pep257_lint_hook.cmake;18;ament_pep257;/opt/ros/rolling/share/ament_cmake_pep257/cmake/ament_cmake_pep257_lint_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;21;ament_execute_extensions;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_package.cmake;66;ament_execute_extensions;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;91;ament_package;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;0;")
add_test(uncrustify "/usr/bin/python3.10" "-u" "/opt/ros/rolling/share/ament_cmake_test/cmake/run_test.py" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/uncrustify.xunit.xml" "--package-name" "action_servers" "--output-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/ament_uncrustify/uncrustify.txt" "--command" "/opt/ros/rolling/bin/ament_uncrustify" "--xunit-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/uncrustify.xunit.xml")
set_tests_properties(uncrustify PROPERTIES  LABELS "uncrustify;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/aaf/ros2_ws/circuits/src/action_servers" _BACKTRACE_TRIPLES "/opt/ros/rolling/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/rolling/share/ament_cmake_uncrustify/cmake/ament_uncrustify.cmake;70;ament_add_test;/opt/ros/rolling/share/ament_cmake_uncrustify/cmake/ament_cmake_uncrustify_lint_hook.cmake;43;ament_uncrustify;/opt/ros/rolling/share/ament_cmake_uncrustify/cmake/ament_cmake_uncrustify_lint_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;21;ament_execute_extensions;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_package.cmake;66;ament_execute_extensions;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;91;ament_package;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;0;")
add_test(xmllint "/usr/bin/python3.10" "-u" "/opt/ros/rolling/share/ament_cmake_test/cmake/run_test.py" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/xmllint.xunit.xml" "--package-name" "action_servers" "--output-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/ament_xmllint/xmllint.txt" "--command" "/opt/ros/rolling/bin/ament_xmllint" "--xunit-file" "/home/aaf/ros2_ws/circuits/src/action_servers/build/action_servers/test_results/action_servers/xmllint.xunit.xml")
set_tests_properties(xmllint PROPERTIES  LABELS "xmllint;linter" TIMEOUT "60" WORKING_DIRECTORY "/home/aaf/ros2_ws/circuits/src/action_servers" _BACKTRACE_TRIPLES "/opt/ros/rolling/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/rolling/share/ament_cmake_xmllint/cmake/ament_xmllint.cmake;50;ament_add_test;/opt/ros/rolling/share/ament_cmake_xmllint/cmake/ament_cmake_xmllint_lint_hook.cmake;18;ament_xmllint;/opt/ros/rolling/share/ament_cmake_xmllint/cmake/ament_cmake_xmllint_lint_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;21;ament_execute_extensions;/opt/ros/rolling/share/ament_lint_auto/cmake/ament_lint_auto_package_hook.cmake;0;;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_execute_extensions.cmake;48;include;/opt/ros/rolling/share/ament_cmake_core/cmake/core/ament_package.cmake;66;ament_execute_extensions;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;91;ament_package;/home/aaf/ros2_ws/circuits/src/action_servers/CMakeLists.txt;0;")
