// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from brain:action/Led21On.idl
// generated code does not contain a copyright notice

#ifndef BRAIN__ACTION__DETAIL__LED21_ON__STRUCT_H_
#define BRAIN__ACTION__DETAIL__LED21_ON__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'turn_on'
#include "rosidl_runtime_c/string.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_Goal
{
  rosidl_runtime_c__String turn_on;
} brain__action__Led21On_Goal;

// Struct for a sequence of brain__action__Led21On_Goal.
typedef struct brain__action__Led21On_Goal__Sequence
{
  brain__action__Led21On_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'confirmed'
// already included above
// #include "rosidl_runtime_c/string.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_Result
{
  rosidl_runtime_c__String confirmed;
} brain__action__Led21On_Result;

// Struct for a sequence of brain__action__Led21On_Result.
typedef struct brain__action__Led21On_Result__Sequence
{
  brain__action__Led21On_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'process_feed'
// already included above
// #include "rosidl_runtime_c/string.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_Feedback
{
  rosidl_runtime_c__String process_feed;
} brain__action__Led21On_Feedback;

// Struct for a sequence of brain__action__Led21On_Feedback.
typedef struct brain__action__Led21On_Feedback__Sequence
{
  brain__action__Led21On_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "brain/action/detail/led21_on__struct.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  brain__action__Led21On_Goal goal;
} brain__action__Led21On_SendGoal_Request;

// Struct for a sequence of brain__action__Led21On_SendGoal_Request.
typedef struct brain__action__Led21On_SendGoal_Request__Sequence
{
  brain__action__Led21On_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} brain__action__Led21On_SendGoal_Response;

// Struct for a sequence of brain__action__Led21On_SendGoal_Response.
typedef struct brain__action__Led21On_SendGoal_Response__Sequence
{
  brain__action__Led21On_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} brain__action__Led21On_GetResult_Request;

// Struct for a sequence of brain__action__Led21On_GetResult_Request.
typedef struct brain__action__Led21On_GetResult_Request__Sequence
{
  brain__action__Led21On_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "brain/action/detail/led21_on__struct.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_GetResult_Response
{
  int8_t status;
  brain__action__Led21On_Result result;
} brain__action__Led21On_GetResult_Response;

// Struct for a sequence of brain__action__Led21On_GetResult_Response.
typedef struct brain__action__Led21On_GetResult_Response__Sequence
{
  brain__action__Led21On_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "brain/action/detail/led21_on__struct.h"

// Struct defined in action/Led21On in the package brain.
typedef struct brain__action__Led21On_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  brain__action__Led21On_Feedback feedback;
} brain__action__Led21On_FeedbackMessage;

// Struct for a sequence of brain__action__Led21On_FeedbackMessage.
typedef struct brain__action__Led21On_FeedbackMessage__Sequence
{
  brain__action__Led21On_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21On_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // BRAIN__ACTION__DETAIL__LED21_ON__STRUCT_H_
