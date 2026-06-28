# Library files
# ==============
set(OPACK_EXAMPLES
	00_empty
	01_sim_controls
	02_world
	03_simple_agent
	04_activity
	05_perception
	06_action
)

set(EXAMPLES_PLECS
	activity.flecs
	perception.flecs
	simple_agent.flecs
)

foreach(example IN LISTS OPACK_EXAMPLES)
	add_executable(${example} "${CMAKE_CURRENT_LIST_DIR}/${example}/main.cpp")
	target_compile_features(${example} PRIVATE cxx_std_20)
	target_link_libraries(${example} PRIVATE opack)
	set_target_properties(${example} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${OPACK_BINARY_OUTPUT_DIR}/examples/${example}/")
	if(EXISTS "${CMAKE_CURRENT_LIST_DIR}/${example}/state.flecs")
		configure_file(
				"${CMAKE_CURRENT_LIST_DIR}/${example}/state.flecs" 
				"${OPACK_BINARY_OUTPUT_DIR}/examples/${example}/state.flecs")
	endif()
endforeach()

foreach(plecs IN LISTS EXAMPLES_PLECS)
endforeach()

# 07_opaca_actor: deterministic actor with SHAKE256 UUID gen (needs OpenSSL)
find_package(OpenSSL REQUIRED)
add_executable(07_opaca_actor "${CMAKE_CURRENT_LIST_DIR}/07_opaca_actor/main.cpp")
target_compile_features(07_opaca_actor PRIVATE cxx_std_20)
target_link_libraries(07_opaca_actor PRIVATE opack OpenSSL::Crypto)
set_target_properties(07_opaca_actor PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${OPACK_BINARY_OUTPUT_DIR}/examples/07_opaca_actor/")

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
	add_executable(FlecsTestBench "test_bench.cpp")
	target_compile_features(FlecsTestBench PRIVATE cxx_std_11)
	target_link_libraries(FlecsTestBench PRIVATE flecs_static)
	set_target_properties(FlecsTestBench PROPERTIES OUTPUT_NAME "FlecsTestBench")
	set_target_properties(FlecsTestBench PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${OPACK_BINARY_OUTPUT_DIR}/")
endif()

