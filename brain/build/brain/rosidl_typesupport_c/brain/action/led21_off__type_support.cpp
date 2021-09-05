// generated from rosidl_typesupport_c/resource/idl__type_support.cpp.em
// with input from brain:action/Led21Off.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "brain/msg/rosidl_typesupport_c__visibility_control.h"
#include "brain/action/detail/led21_off__struct.h"
#include "rosidl_typesupport_c/identifier.h"
#include "rosidl_typesupport_c/message_type_support_dispatch.h"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_c/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_Goal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_Goal_type_support_ids_t;

static const _Led21Off_Goal_type_support_ids_t _Led21Off_Goal_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_Goal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_Goal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_Goal_type_support_symbol_names_t _Led21Off_Goal_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_Goal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_Goal)),
  }
};

typedef struct _Led21Off_Goal_type_support_data_t
{
  void * data[2];
} _Led21Off_Goal_type_support_data_t;

static _Led21Off_Goal_type_support_data_t _Led21Off_Goal_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_Goal_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_Goal_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_Goal_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_Goal_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_Goal_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_Goal_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_Goal)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_Goal_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_Result_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_Result_type_support_ids_t;

static const _Led21Off_Result_type_support_ids_t _Led21Off_Result_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_Result_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_Result_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_Result_type_support_symbol_names_t _Led21Off_Result_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_Result)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_Result)),
  }
};

typedef struct _Led21Off_Result_type_support_data_t
{
  void * data[2];
} _Led21Off_Result_type_support_data_t;

static _Led21Off_Result_type_support_data_t _Led21Off_Result_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_Result_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_Result_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_Result_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_Result_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_Result_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_Result_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_Result)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_Result_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_Feedback_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_Feedback_type_support_ids_t;

static const _Led21Off_Feedback_type_support_ids_t _Led21Off_Feedback_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_Feedback_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_Feedback_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_Feedback_type_support_symbol_names_t _Led21Off_Feedback_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_Feedback)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_Feedback)),
  }
};

typedef struct _Led21Off_Feedback_type_support_data_t
{
  void * data[2];
} _Led21Off_Feedback_type_support_data_t;

static _Led21Off_Feedback_type_support_data_t _Led21Off_Feedback_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_Feedback_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_Feedback_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_Feedback_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_Feedback_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_Feedback_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_Feedback_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_Feedback)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_SendGoal_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_SendGoal_Request_type_support_ids_t;

static const _Led21Off_SendGoal_Request_type_support_ids_t _Led21Off_SendGoal_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_SendGoal_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_SendGoal_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_SendGoal_Request_type_support_symbol_names_t _Led21Off_SendGoal_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_SendGoal_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_SendGoal_Request)),
  }
};

typedef struct _Led21Off_SendGoal_Request_type_support_data_t
{
  void * data[2];
} _Led21Off_SendGoal_Request_type_support_data_t;

static _Led21Off_SendGoal_Request_type_support_data_t _Led21Off_SendGoal_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_SendGoal_Request_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_SendGoal_Request_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_SendGoal_Request_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_SendGoal_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_SendGoal_Request_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_SendGoal_Request_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_SendGoal_Request)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_SendGoal_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_SendGoal_Response_type_support_ids_t;

static const _Led21Off_SendGoal_Response_type_support_ids_t _Led21Off_SendGoal_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_SendGoal_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_SendGoal_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_SendGoal_Response_type_support_symbol_names_t _Led21Off_SendGoal_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_SendGoal_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_SendGoal_Response)),
  }
};

typedef struct _Led21Off_SendGoal_Response_type_support_data_t
{
  void * data[2];
} _Led21Off_SendGoal_Response_type_support_data_t;

static _Led21Off_SendGoal_Response_type_support_data_t _Led21Off_SendGoal_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_SendGoal_Response_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_SendGoal_Response_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_SendGoal_Response_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_SendGoal_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_SendGoal_Response_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_SendGoal_Response_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_SendGoal_Response)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
#include "rosidl_typesupport_c/service_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_SendGoal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_SendGoal_type_support_ids_t;

static const _Led21Off_SendGoal_type_support_ids_t _Led21Off_SendGoal_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_SendGoal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_SendGoal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_SendGoal_type_support_symbol_names_t _Led21Off_SendGoal_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_SendGoal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_SendGoal)),
  }
};

typedef struct _Led21Off_SendGoal_type_support_data_t
{
  void * data[2];
} _Led21Off_SendGoal_type_support_data_t;

static _Led21Off_SendGoal_type_support_data_t _Led21Off_SendGoal_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_SendGoal_service_typesupport_map = {
  2,
  "brain",
  &_Led21Off_SendGoal_service_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_SendGoal_service_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_SendGoal_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t Led21Off_SendGoal_service_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_SendGoal_service_typesupport_map),
  rosidl_typesupport_c__get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_SendGoal)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_SendGoal_service_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_GetResult_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_GetResult_Request_type_support_ids_t;

static const _Led21Off_GetResult_Request_type_support_ids_t _Led21Off_GetResult_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_GetResult_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_GetResult_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_GetResult_Request_type_support_symbol_names_t _Led21Off_GetResult_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_GetResult_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_GetResult_Request)),
  }
};

typedef struct _Led21Off_GetResult_Request_type_support_data_t
{
  void * data[2];
} _Led21Off_GetResult_Request_type_support_data_t;

static _Led21Off_GetResult_Request_type_support_data_t _Led21Off_GetResult_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_GetResult_Request_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_GetResult_Request_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_GetResult_Request_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_GetResult_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_GetResult_Request_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_GetResult_Request_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_GetResult_Request)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_GetResult_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_GetResult_Response_type_support_ids_t;

static const _Led21Off_GetResult_Response_type_support_ids_t _Led21Off_GetResult_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_GetResult_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_GetResult_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_GetResult_Response_type_support_symbol_names_t _Led21Off_GetResult_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_GetResult_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_GetResult_Response)),
  }
};

typedef struct _Led21Off_GetResult_Response_type_support_data_t
{
  void * data[2];
} _Led21Off_GetResult_Response_type_support_data_t;

static _Led21Off_GetResult_Response_type_support_data_t _Led21Off_GetResult_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_GetResult_Response_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_GetResult_Response_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_GetResult_Response_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_GetResult_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_GetResult_Response_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_GetResult_Response_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_GetResult_Response)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/service_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_GetResult_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_GetResult_type_support_ids_t;

static const _Led21Off_GetResult_type_support_ids_t _Led21Off_GetResult_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_GetResult_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_GetResult_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_GetResult_type_support_symbol_names_t _Led21Off_GetResult_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_GetResult)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_GetResult)),
  }
};

typedef struct _Led21Off_GetResult_type_support_data_t
{
  void * data[2];
} _Led21Off_GetResult_type_support_data_t;

static _Led21Off_GetResult_type_support_data_t _Led21Off_GetResult_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_GetResult_service_typesupport_map = {
  2,
  "brain",
  &_Led21Off_GetResult_service_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_GetResult_service_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_GetResult_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t Led21Off_GetResult_service_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_GetResult_service_typesupport_map),
  rosidl_typesupport_c__get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_GetResult)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_GetResult_service_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "brain/msg/rosidl_typesupport_c__visibility_control.h"
// already included above
// #include "brain/action/detail/led21_off__struct.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace brain
{

namespace action
{

namespace rosidl_typesupport_c
{

typedef struct _Led21Off_FeedbackMessage_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _Led21Off_FeedbackMessage_type_support_ids_t;

static const _Led21Off_FeedbackMessage_type_support_ids_t _Led21Off_FeedbackMessage_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _Led21Off_FeedbackMessage_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _Led21Off_FeedbackMessage_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _Led21Off_FeedbackMessage_type_support_symbol_names_t _Led21Off_FeedbackMessage_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, brain, action, Led21Off_FeedbackMessage)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, brain, action, Led21Off_FeedbackMessage)),
  }
};

typedef struct _Led21Off_FeedbackMessage_type_support_data_t
{
  void * data[2];
} _Led21Off_FeedbackMessage_type_support_data_t;

static _Led21Off_FeedbackMessage_type_support_data_t _Led21Off_FeedbackMessage_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _Led21Off_FeedbackMessage_message_typesupport_map = {
  2,
  "brain",
  &_Led21Off_FeedbackMessage_message_typesupport_ids.typesupport_identifier[0],
  &_Led21Off_FeedbackMessage_message_typesupport_symbol_names.symbol_name[0],
  &_Led21Off_FeedbackMessage_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t Led21Off_FeedbackMessage_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_Led21Off_FeedbackMessage_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_c

}  // namespace action

}  // namespace brain

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, brain, action, Led21Off_FeedbackMessage)() {
  return &::brain::action::rosidl_typesupport_c::Led21Off_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

#include "action_msgs/msg/goal_status_array.h"
#include "action_msgs/srv/cancel_goal.h"
#include "brain/action/led21_off.h"
#include "brain/action/detail/led21_off__type_support.h"

static rosidl_action_type_support_t _brain__action__Led21Off__typesupport_c;

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_C_EXPORT_brain
const rosidl_action_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__ACTION_SYMBOL_NAME(
  rosidl_typesupport_c, brain, action, Led21Off)()
{
  // Thread-safe by always writing the same values to the static struct
  _brain__action__Led21Off__typesupport_c.goal_service_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(
    rosidl_typesupport_c, brain, action, Led21Off_SendGoal)();
  _brain__action__Led21Off__typesupport_c.result_service_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(
    rosidl_typesupport_c, brain, action, Led21Off_GetResult)();
  _brain__action__Led21Off__typesupport_c.cancel_service_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(
    rosidl_typesupport_c, action_msgs, srv, CancelGoal)();
  _brain__action__Led21Off__typesupport_c.feedback_message_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c, brain, action, Led21Off_FeedbackMessage)();
  _brain__action__Led21Off__typesupport_c.status_message_type_support =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c, action_msgs, msg, GoalStatusArray)();

  return &_brain__action__Led21Off__typesupport_c;
}

#ifdef __cplusplus
}
#endif
