#ifndef ACTION_SERVERS__VISIBILITY_CONTROL_H_
#define ACTION_SERVERS__VISIBILITY_CONTROL_H_

#ifdef __cplusplus
extern "C"
{
#endif

// This logic was borrowed (then namespaced) from the examples on the gcc wiki:
//     https://gcc.gnu.org/wiki/Visibility

#if defined _WIN32 || defined __CYGWIN__
  #ifdef __GNUC__
    #define ACTION_SERVERS_EXPORT __attribute__ ((dllexport))
    #define ACTION_SERVERS_IMPORT __attribute__ ((dllimport))
  #else
    #define ACTION_SERVERS_EXPORT __declspec(dllexport)
    #define ACTION_SERVERS_IMPORT __declspec(dllimport)
  #endif
  #ifdef ACTION_SERVERS_BUILDING_DLL
    #define ACTION_SERVERS_PUBLIC ACTION_SERVERS_EXPORT
  #else
    #define ACTION_SERVERS_PUBLIC ACTION_SERVERS_IMPORT
  #endif
  #define ACTION_SERVERS_PUBLIC_TYPE ACTION_SERVERS_PUBLIC
  #define ACTION_SERVERS_LOCAL
#else
  #define ACTION_SERVERS_EXPORT __attribute__ ((visibility("default")))
  #define ACTION_SERVERS_IMPORT
  #if __GNUC__ >= 4
    #define ACTION_SERVERS_PUBLIC __attribute__ ((visibility("default")))
    #define ACTION_SERVERS_LOCAL  __attribute__ ((visibility("hidden")))
  #else
    #define ACTION_SERVERS_PUBLIC
    #define ACTION_SERVERS_LOCAL
  #endif
  #define ACTION_SERVERS_PUBLIC_TYPE
#endif

#ifdef __cplusplus
}
#endif

#endif  // ACTION_SERVERS__VISIBILITY_CONTROL_H_
