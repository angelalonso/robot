// generated from rosidl_typesupport_cpp/resource/idl__type_support.cpp.em
// with input from brain:action/Led.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "brain/action/detail/led__struct.hpp"
#include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
#include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_Goal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_Goal_type_support_ids_t;

static const _Led_Goal_type_support_ids_t _Led_Goal_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_Goal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_Goal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_Goal_type_support_symbol_names_t _Led_Goal_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_Goal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_Goal)),
  }
};

typedef struct _Led_Goal_type_support_data_t
{
  void * data[2];
} _Led_Goal_type_support_data_t;

static _Led_Goal_type_support_data_t _Led_Goal_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_Goal_message_typesupport_map = {
  2,
  "brain",
  &_Led_Goal_message_typesupport_ids.typesupport_identifier[0],
  &_Led_Goal_message_typesupport_symbol_names.symbol_name[0],
  &_Led_Goal_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_Goal_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_Goal_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_Goal>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_Goal_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_Goal)() {
  return get_message_type_support_handle<brain::action::Led_Goal>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_Result_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_Result_type_support_ids_t;

static const _Led_Result_type_support_ids_t _Led_Result_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_Result_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_Result_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_Result_type_support_symbol_names_t _Led_Result_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_Result)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_Result)),
  }
};

typedef struct _Led_Result_type_support_data_t
{
  void * data[2];
} _Led_Result_type_support_data_t;

static _Led_Result_type_support_data_t _Led_Result_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_Result_message_typesupport_map = {
  2,
  "brain",
  &_Led_Result_message_typesupport_ids.typesupport_identifier[0],
  &_Led_Result_message_typesupport_symbol_names.symbol_name[0],
  &_Led_Result_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_Result_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_Result_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_Result>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_Result_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_Result)() {
  return get_message_type_support_handle<brain::action::Led_Result>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_Feedback_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_Feedback_type_support_ids_t;

static const _Led_Feedback_type_support_ids_t _Led_Feedback_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_Feedback_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_Feedback_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_Feedback_type_support_symbol_names_t _Led_Feedback_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_Feedback)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_Feedback)),
  }
};

typedef struct _Led_Feedback_type_support_data_t
{
  void * data[2];
} _Led_Feedback_type_support_data_t;

static _Led_Feedback_type_support_data_t _Led_Feedback_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_Feedback_message_typesupport_map = {
  2,
  "brain",
  &_Led_Feedback_message_typesupport_ids.typesupport_identifier[0],
  &_Led_Feedback_message_typesupport_symbol_names.symbol_name[0],
  &_Led_Feedback_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_Feedback_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_Feedback_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_Feedback>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_Feedback)() {
  return get_message_type_support_handle<brain::action::Led_Feedback>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_SendGoal_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_SendGoal_Request_type_support_ids_t;

static const _Led_SendGoal_Request_type_support_ids_t _Led_SendGoal_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_SendGoal_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_SendGoal_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_SendGoal_Request_type_support_symbol_names_t _Led_SendGoal_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_SendGoal_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_SendGoal_Request)),
  }
};

typedef struct _Led_SendGoal_Request_type_support_data_t
{
  void * data[2];
} _Led_SendGoal_Request_type_support_data_t;

static _Led_SendGoal_Request_type_support_data_t _Led_SendGoal_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_SendGoal_Request_message_typesupport_map = {
  2,
  "brain",
  &_Led_SendGoal_Request_message_typesupport_ids.typesupport_identifier[0],
  &_Led_SendGoal_Request_message_typesupport_symbol_names.symbol_name[0],
  &_Led_SendGoal_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_SendGoal_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_SendGoal_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_SendGoal_Request>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_SendGoal_Request)() {
  return get_message_type_support_handle<brain::action::Led_SendGoal_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_SendGoal_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_SendGoal_Response_type_support_ids_t;

static const _Led_SendGoal_Response_type_support_ids_t _Led_SendGoal_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_SendGoal_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_SendGoal_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_SendGoal_Response_type_support_symbol_names_t _Led_SendGoal_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_SendGoal_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_SendGoal_Response)),
  }
};

typedef struct _Led_SendGoal_Response_type_support_data_t
{
  void * data[2];
} _Led_SendGoal_Response_type_support_data_t;

static _Led_SendGoal_Response_type_support_data_t _Led_SendGoal_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_SendGoal_Response_message_typesupport_map = {
  2,
  "brain",
  &_Led_SendGoal_Response_message_typesupport_ids.typesupport_identifier[0],
  &_Led_SendGoal_Response_message_typesupport_symbol_names.symbol_name[0],
  &_Led_SendGoal_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_SendGoal_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_SendGoal_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_SendGoal_Response>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_SendGoal_Response)() {
  return get_message_type_support_handle<brain::action::Led_SendGoal_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_SendGoal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_SendGoal_type_support_ids_t;

static const _Led_SendGoal_type_support_ids_t _Led_SendGoal_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_SendGoal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_SendGoal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_SendGoal_type_support_symbol_names_t _Led_SendGoal_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_SendGoal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_SendGoal)),
  }
};

typedef struct _Led_SendGoal_type_support_data_t
{
  void * data[2];
} _Led_SendGoal_type_support_data_t;

static _Led_SendGoal_type_support_data_t _Led_SendGoal_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_SendGoal_service_typesupport_map = {
  2,
  "brain",
  &_Led_SendGoal_service_typesupport_ids.typesupport_identifier[0],
  &_Led_SendGoal_service_typesupport_symbol_names.symbol_name[0],
  &_Led_SendGoal_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t Led_SendGoal_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_SendGoal_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<brain::action::Led_SendGoal>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_SendGoal_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_GetResult_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_GetResult_Request_type_support_ids_t;

static const _Led_GetResult_Request_type_support_ids_t _Led_GetResult_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_GetResult_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_GetResult_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_GetResult_Request_type_support_symbol_names_t _Led_GetResult_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_GetResult_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_GetResult_Request)),
  }
};

typedef struct _Led_GetResult_Request_type_support_data_t
{
  void * data[2];
} _Led_GetResult_Request_type_support_data_t;

static _Led_GetResult_Request_type_support_data_t _Led_GetResult_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_GetResult_Request_message_typesupport_map = {
  2,
  "brain",
  &_Led_GetResult_Request_message_typesupport_ids.typesupport_identifier[0],
  &_Led_GetResult_Request_message_typesupport_symbol_names.symbol_name[0],
  &_Led_GetResult_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_GetResult_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_GetResult_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_GetResult_Request>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_GetResult_Request)() {
  return get_message_type_support_handle<brain::action::Led_GetResult_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_GetResult_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_GetResult_Response_type_support_ids_t;

static const _Led_GetResult_Response_type_support_ids_t _Led_GetResult_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_GetResult_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_GetResult_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_GetResult_Response_type_support_symbol_names_t _Led_GetResult_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_GetResult_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_GetResult_Response)),
  }
};

typedef struct _Led_GetResult_Response_type_support_data_t
{
  void * data[2];
} _Led_GetResult_Response_type_support_data_t;

static _Led_GetResult_Response_type_support_data_t _Led_GetResult_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_GetResult_Response_message_typesupport_map = {
  2,
  "brain",
  &_Led_GetResult_Response_message_typesupport_ids.typesupport_identifier[0],
  &_Led_GetResult_Response_message_typesupport_symbol_names.symbol_name[0],
  &_Led_GetResult_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_GetResult_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_GetResult_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_GetResult_Response>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_GetResult_Response)() {
  return get_message_type_support_handle<brain::action::Led_GetResult_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_GetResult_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_GetResult_type_support_ids_t;

static const _Led_GetResult_type_support_ids_t _Led_GetResult_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_GetResult_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_GetResult_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_GetResult_type_support_symbol_names_t _Led_GetResult_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_GetResult)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_GetResult)),
  }
};

typedef struct _Led_GetResult_type_support_data_t
{
  void * data[2];
} _Led_GetResult_type_support_data_t;

static _Led_GetResult_type_support_data_t _Led_GetResult_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_GetResult_service_typesupport_map = {
  2,
  "brain",
  &_Led_GetResult_service_typesupport_ids.typesupport_identifier[0],
  &_Led_GetResult_service_typesupport_symbol_names.symbol_name[0],
  &_Led_GetResult_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t Led_GetResult_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_GetResult_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<brain::action::Led_GetResult>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_GetResult_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _Led_FeedbackMessage_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led_FeedbackMessage_type_support_ids_t;

static const _Led_FeedbackMessage_type_support_ids_t _Led_FeedbackMessage_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _Led_FeedbackMessage_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led_FeedbackMessage_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led_FeedbackMessage_type_support_symbol_names_t _Led_FeedbackMessage_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, brain, action, Led_FeedbackMessage)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, brain, action, Led_FeedbackMessage)),
  }
};

typedef struct _Led_FeedbackMessage_type_support_data_t
{
  void * data[2];
} _Led_FeedbackMessage_type_support_data_t;

static _Led_FeedbackMessage_type_support_data_t _Led_FeedbackMessage_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led_FeedbackMessage_message_typesupport_map = {
  2,
  "brain",
  &_Led_FeedbackMessage_message_typesupport_ids.typesupport_identifier[0],
  &_Led_FeedbackMessage_message_typesupport_symbol_names.symbol_name[0],
  &_Led_FeedbackMessage_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led_FeedbackMessage_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led_FeedbackMessage_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<brain::action::Led_FeedbackMessage>()
{
  return &::brain::action::rosidl_typesupport_cpp::Led_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, brain, action, Led_FeedbackMessage)() {
  return get_message_type_support_handle<brain::action::Led_FeedbackMessage>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

#include "action_msgs/msg/goal_status_array.hpp"
#include "action_msgs/srv/cancel_goal.hpp"
// already included above
// #include "brain/action/detail/led__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_runtime_c/action_type_support_struct.h"
#include "rosidl_typesupport_cpp/action_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_cpp
{

static rosidl_action_type_support_t Led_action_type_support_handle = {
  NULL, NULL, NULL, NULL, NULL};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace brain

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
get_action_type_support_handle<brain::action::Led>()
{
  using ::brain::action::rosidl_typesupport_cpp::Led_action_type_support_handle;
  // Thread-safe by always writing the same values to the static struct
  Led_action_type_support_handle.goal_service_type_support = get_service_type_support_handle<::brain::action::Led::Impl::SendGoalService>();
  Led_action_type_support_handle.result_service_type_support = get_service_type_support_handle<::brain::action::Led::Impl::GetResultService>();
  Led_action_type_support_handle.cancel_service_type_support = get_service_type_support_handle<::brain::action::Led::Impl::CancelGoalService>();
  Led_action_type_support_handle.feedback_message_type_support = get_message_type_support_handle<::brain::action::Led::Impl::FeedbackMessage>();
  Led_action_type_support_handle.status_message_type_support = get_message_type_support_handle<::brain::action::Led::Impl::GoalStatusMessage>();
  return &Led_action_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp
