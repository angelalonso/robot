// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from brain:action/Led21.idl
// generated code does not contain a copyright notice

#ifndef BRAIN__ACTION__DETAIL__LED21__STRUCT_H_
#define BRAIN__ACTION__DETAIL__LED21__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_Goal
{
  bool turn_on;
} brain__action__Led21_Goal;

// Struct for a sequence of brain__action__Led21_Goal.
typedef struct brain__action__Led21_Goal__Sequence
{
  brain__action__Led21_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'confirmed'
#include "rosidl_runtime_c/string.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_Result
{
  rosidl_runtime_c__String confirmed;
} brain__action__Led21_Result;

// Struct for a sequence of brain__action__Led21_Result.
typedef struct brain__action__Led21_Result__Sequence
{
  brain__action__Led21_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'process_feed'
// already included above
// #include "rosidl_runtime_c/string.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_Feedback
{
  rosidl_runtime_c__String process_feed;
} brain__action__Led21_Feedback;

// Struct for a sequence of brain__action__Led21_Feedback.
typedef struct brain__action__Led21_Feedback__Sequence
{
  brain__action__Led21_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "brain/action/detail/led21__struct.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  brain__action__Led21_Goal goal;
} brain__action__Led21_SendGoal_Request;

// Struct for a sequence of brain__action__Led21_SendGoal_Request.
typedef struct brain__action__Led21_SendGoal_Request__Sequence
{
  brain__action__Led21_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} brain__action__Led21_SendGoal_Response;

// Struct for a sequence of brain__action__Led21_SendGoal_Response.
typedef struct brain__action__Led21_SendGoal_Response__Sequence
{
  brain__action__Led21_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} brain__action__Led21_GetResult_Request;

// Struct for a sequence of brain__action__Led21_GetResult_Request.
typedef struct brain__action__Led21_GetResult_Request__Sequence
{
  brain__action__Led21_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "brain/action/detail/led21__struct.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_GetResult_Response
{
  int8_t status;
  brain__action__Led21_Result result;
} brain__action__Led21_GetResult_Response;

// Struct for a sequence of brain__action__Led21_GetResult_Response.
typedef struct brain__action__Led21_GetResult_Response__Sequence
{
  brain__action__Led21_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "brain/action/detail/led21__struct.h"

// Struct defined in action/Led21 in the package brain.
typedef struct brain__action__Led21_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  brain__action__Led21_Feedback feedback;
} brain__action__Led21_FeedbackMessage;

// Struct for a sequence of brain__action__Led21_FeedbackMessage.
typedef struct brain__action__Led21_FeedbackMessage__Sequence
{
  brain__action__Led21_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} brain__action__Led21_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // BRAIN__ACTION__DETAIL__LED21__STRUCT_H_
