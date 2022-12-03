// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from action_interfaces:action/Led.idl
// generated code does not contain a copyright notice

#ifndef ACTION_INTERFACES__ACTION__DETAIL__LED__STRUCT_H_
#define ACTION_INTERFACES__ACTION__DETAIL__LED__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_Goal
{
  bool turn_on;
} action_interfaces__action__Led_Goal;

// Struct for a sequence of action_interfaces__action__Led_Goal.
typedef struct action_interfaces__action__Led_Goal__Sequence
{
  action_interfaces__action__Led_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'confirmed'
#include "rosidl_runtime_c/string.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_Result
{
  rosidl_runtime_c__String confirmed;
} action_interfaces__action__Led_Result;

// Struct for a sequence of action_interfaces__action__Led_Result.
typedef struct action_interfaces__action__Led_Result__Sequence
{
  action_interfaces__action__Led_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'process_feed'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_Feedback
{
  rosidl_runtime_c__String process_feed;
} action_interfaces__action__Led_Feedback;

// Struct for a sequence of action_interfaces__action__Led_Feedback.
typedef struct action_interfaces__action__Led_Feedback__Sequence
{
  action_interfaces__action__Led_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "action_interfaces/action/detail/led__struct.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  action_interfaces__action__Led_Goal goal;
} action_interfaces__action__Led_SendGoal_Request;

// Struct for a sequence of action_interfaces__action__Led_SendGoal_Request.
typedef struct action_interfaces__action__Led_SendGoal_Request__Sequence
{
  action_interfaces__action__Led_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} action_interfaces__action__Led_SendGoal_Response;

// Struct for a sequence of action_interfaces__action__Led_SendGoal_Response.
typedef struct action_interfaces__action__Led_SendGoal_Response__Sequence
{
  action_interfaces__action__Led_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} action_interfaces__action__Led_GetResult_Request;

// Struct for a sequence of action_interfaces__action__Led_GetResult_Request.
typedef struct action_interfaces__action__Led_GetResult_Request__Sequence
{
  action_interfaces__action__Led_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "action_interfaces/action/detail/led__struct.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_GetResult_Response
{
  int8_t status;
  action_interfaces__action__Led_Result result;
} action_interfaces__action__Led_GetResult_Response;

// Struct for a sequence of action_interfaces__action__Led_GetResult_Response.
typedef struct action_interfaces__action__Led_GetResult_Response__Sequence
{
  action_interfaces__action__Led_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "action_interfaces/action/detail/led__struct.h"

/// Struct defined in action/Led in the package action_interfaces.
typedef struct action_interfaces__action__Led_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  action_interfaces__action__Led_Feedback feedback;
} action_interfaces__action__Led_FeedbackMessage;

// Struct for a sequence of action_interfaces__action__Led_FeedbackMessage.
typedef struct action_interfaces__action__Led_FeedbackMessage__Sequence
{
  action_interfaces__action__Led_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_interfaces__action__Led_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ACTION_INTERFACES__ACTION__DETAIL__LED__STRUCT_H_