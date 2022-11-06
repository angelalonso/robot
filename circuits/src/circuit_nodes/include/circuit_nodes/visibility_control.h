#ifndef CIRCUIT_NODES__VISIBILITY_CONTROL_H_
#define CIRCUIT_NODES__VISIBILITY_CONTROL_H_

#ifdef __cplusplus
extern "C"
{
#endif

// This logic was borrowed (then namespaced) from the examples on the gcc wiki:
//     https://gcc.gnu.org/wiki/Visibility

#if defined _WIN32 || defined __CYGWIN__
  #ifdef __GNUC__
    #define CIRCUIT_NODES_EXPORT __attribute__ ((dllexport))
    #define CIRCUIT_NODES_IMPORT __attribute__ ((dllimport))
  #else
    #define CIRCUIT_NODES_EXPORT __declspec(dllexport)
    #define CIRCUIT_NODES_IMPORT __declspec(dllimport)
  #endif
  #ifdef CIRCUIT_NODES_BUILDING_DLL
    #define CIRCUIT_NODES_PUBLIC CIRCUIT_NODES_EXPORT
  #else
    #define CIRCUIT_NODES_PUBLIC CIRCUIT_NODES_IMPORT
  #endif
  #define CIRCUIT_NODES_PUBLIC_TYPE CIRCUIT_NODES_PUBLIC
  #define CIRCUIT_NODES_LOCAL
#else
  #define CIRCUIT_NODES_EXPORT __attribute__ ((visibility("default")))
  #define CIRCUIT_NODES_IMPORT
  #if __GNUC__ >= 4
    #define CIRCUIT_NODES_PUBLIC __attribute__ ((visibility("default")))
    #define CIRCUIT_NODES_LOCAL  __attribute__ ((visibility("hidden")))
  #else
    #define CIRCUIT_NODES_PUBLIC
    #define CIRCUIT_NODES_LOCAL
  #endif
  #define CIRCUIT_NODES_PUBLIC_TYPE
#endif

#ifdef __cplusplus
}
#endif

#endif  // CIRCUIT_NODES__VISIBILITY_CONTROL_H_
