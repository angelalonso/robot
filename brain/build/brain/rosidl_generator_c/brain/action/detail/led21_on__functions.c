// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from brain:action/Led21On.idl
// generated code does not contain a copyright notice
#include "brain/action/detail/led21_on__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `turn_on`
#include "rosidl_runtime_c/string_functions.h"

bool
brain__action__Led21On_Goal__init(brain__action__Led21On_Goal * msg)
{
  if (!msg) {
    return false;
  }
  // turn_on
  if (!rosidl_runtime_c__String__init(&msg->turn_on)) {
    brain__action__Led21On_Goal__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_Goal__fini(brain__action__Led21On_Goal * msg)
{
  if (!msg) {
    return;
  }
  // turn_on
  rosidl_runtime_c__String__fini(&msg->turn_on);
}

brain__action__Led21On_Goal *
brain__action__Led21On_Goal__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Goal * msg = (brain__action__Led21On_Goal *)allocator.allocate(sizeof(brain__action__Led21On_Goal), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_Goal));
  bool success = brain__action__Led21On_Goal__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_Goal__destroy(brain__action__Led21On_Goal * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_Goal__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_Goal__Sequence__init(brain__action__Led21On_Goal__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Goal * data = NULL;

  if (size) {
    data = (brain__action__Led21On_Goal *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_Goal), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_Goal__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_Goal__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_Goal__Sequence__fini(brain__action__Led21On_Goal__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_Goal__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_Goal__Sequence *
brain__action__Led21On_Goal__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Goal__Sequence * array = (brain__action__Led21On_Goal__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_Goal__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_Goal__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_Goal__Sequence__destroy(brain__action__Led21On_Goal__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_Goal__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `confirmed`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

bool
brain__action__Led21On_Result__init(brain__action__Led21On_Result * msg)
{
  if (!msg) {
    return false;
  }
  // confirmed
  if (!rosidl_runtime_c__String__init(&msg->confirmed)) {
    brain__action__Led21On_Result__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_Result__fini(brain__action__Led21On_Result * msg)
{
  if (!msg) {
    return;
  }
  // confirmed
  rosidl_runtime_c__String__fini(&msg->confirmed);
}

brain__action__Led21On_Result *
brain__action__Led21On_Result__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Result * msg = (brain__action__Led21On_Result *)allocator.allocate(sizeof(brain__action__Led21On_Result), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_Result));
  bool success = brain__action__Led21On_Result__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_Result__destroy(brain__action__Led21On_Result * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_Result__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_Result__Sequence__init(brain__action__Led21On_Result__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Result * data = NULL;

  if (size) {
    data = (brain__action__Led21On_Result *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_Result), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_Result__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_Result__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_Result__Sequence__fini(brain__action__Led21On_Result__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_Result__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_Result__Sequence *
brain__action__Led21On_Result__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Result__Sequence * array = (brain__action__Led21On_Result__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_Result__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_Result__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_Result__Sequence__destroy(brain__action__Led21On_Result__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_Result__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `process_feed`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

bool
brain__action__Led21On_Feedback__init(brain__action__Led21On_Feedback * msg)
{
  if (!msg) {
    return false;
  }
  // process_feed
  if (!rosidl_runtime_c__String__init(&msg->process_feed)) {
    brain__action__Led21On_Feedback__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_Feedback__fini(brain__action__Led21On_Feedback * msg)
{
  if (!msg) {
    return;
  }
  // process_feed
  rosidl_runtime_c__String__fini(&msg->process_feed);
}

brain__action__Led21On_Feedback *
brain__action__Led21On_Feedback__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Feedback * msg = (brain__action__Led21On_Feedback *)allocator.allocate(sizeof(brain__action__Led21On_Feedback), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_Feedback));
  bool success = brain__action__Led21On_Feedback__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_Feedback__destroy(brain__action__Led21On_Feedback * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_Feedback__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_Feedback__Sequence__init(brain__action__Led21On_Feedback__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Feedback * data = NULL;

  if (size) {
    data = (brain__action__Led21On_Feedback *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_Feedback), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_Feedback__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_Feedback__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_Feedback__Sequence__fini(brain__action__Led21On_Feedback__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_Feedback__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_Feedback__Sequence *
brain__action__Led21On_Feedback__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_Feedback__Sequence * array = (brain__action__Led21On_Feedback__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_Feedback__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_Feedback__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_Feedback__Sequence__destroy(brain__action__Led21On_Feedback__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_Feedback__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `goal_id`
#include "unique_identifier_msgs/msg/detail/uuid__functions.h"
// Member `goal`
// already included above
// #include "brain/action/detail/led21_on__functions.h"

bool
brain__action__Led21On_SendGoal_Request__init(brain__action__Led21On_SendGoal_Request * msg)
{
  if (!msg) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__init(&msg->goal_id)) {
    brain__action__Led21On_SendGoal_Request__fini(msg);
    return false;
  }
  // goal
  if (!brain__action__Led21On_Goal__init(&msg->goal)) {
    brain__action__Led21On_SendGoal_Request__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_SendGoal_Request__fini(brain__action__Led21On_SendGoal_Request * msg)
{
  if (!msg) {
    return;
  }
  // goal_id
  unique_identifier_msgs__msg__UUID__fini(&msg->goal_id);
  // goal
  brain__action__Led21On_Goal__fini(&msg->goal);
}

brain__action__Led21On_SendGoal_Request *
brain__action__Led21On_SendGoal_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_SendGoal_Request * msg = (brain__action__Led21On_SendGoal_Request *)allocator.allocate(sizeof(brain__action__Led21On_SendGoal_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_SendGoal_Request));
  bool success = brain__action__Led21On_SendGoal_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_SendGoal_Request__destroy(brain__action__Led21On_SendGoal_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_SendGoal_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_SendGoal_Request__Sequence__init(brain__action__Led21On_SendGoal_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_SendGoal_Request * data = NULL;

  if (size) {
    data = (brain__action__Led21On_SendGoal_Request *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_SendGoal_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_SendGoal_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_SendGoal_Request__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_SendGoal_Request__Sequence__fini(brain__action__Led21On_SendGoal_Request__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_SendGoal_Request__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_SendGoal_Request__Sequence *
brain__action__Led21On_SendGoal_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_SendGoal_Request__Sequence * array = (brain__action__Led21On_SendGoal_Request__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_SendGoal_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_SendGoal_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_SendGoal_Request__Sequence__destroy(brain__action__Led21On_SendGoal_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_SendGoal_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `stamp`
#include "builtin_interfaces/msg/detail/time__functions.h"

bool
brain__action__Led21On_SendGoal_Response__init(brain__action__Led21On_SendGoal_Response * msg)
{
  if (!msg) {
    return false;
  }
  // accepted
  // stamp
  if (!builtin_interfaces__msg__Time__init(&msg->stamp)) {
    brain__action__Led21On_SendGoal_Response__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_SendGoal_Response__fini(brain__action__Led21On_SendGoal_Response * msg)
{
  if (!msg) {
    return;
  }
  // accepted
  // stamp
  builtin_interfaces__msg__Time__fini(&msg->stamp);
}

brain__action__Led21On_SendGoal_Response *
brain__action__Led21On_SendGoal_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_SendGoal_Response * msg = (brain__action__Led21On_SendGoal_Response *)allocator.allocate(sizeof(brain__action__Led21On_SendGoal_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_SendGoal_Response));
  bool success = brain__action__Led21On_SendGoal_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_SendGoal_Response__destroy(brain__action__Led21On_SendGoal_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_SendGoal_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_SendGoal_Response__Sequence__init(brain__action__Led21On_SendGoal_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_SendGoal_Response * data = NULL;

  if (size) {
    data = (brain__action__Led21On_SendGoal_Response *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_SendGoal_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_SendGoal_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_SendGoal_Response__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_SendGoal_Response__Sequence__fini(brain__action__Led21On_SendGoal_Response__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_SendGoal_Response__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_SendGoal_Response__Sequence *
brain__action__Led21On_SendGoal_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_SendGoal_Response__Sequence * array = (brain__action__Led21On_SendGoal_Response__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_SendGoal_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_SendGoal_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_SendGoal_Response__Sequence__destroy(brain__action__Led21On_SendGoal_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_SendGoal_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__functions.h"

bool
brain__action__Led21On_GetResult_Request__init(brain__action__Led21On_GetResult_Request * msg)
{
  if (!msg) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__init(&msg->goal_id)) {
    brain__action__Led21On_GetResult_Request__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_GetResult_Request__fini(brain__action__Led21On_GetResult_Request * msg)
{
  if (!msg) {
    return;
  }
  // goal_id
  unique_identifier_msgs__msg__UUID__fini(&msg->goal_id);
}

brain__action__Led21On_GetResult_Request *
brain__action__Led21On_GetResult_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_GetResult_Request * msg = (brain__action__Led21On_GetResult_Request *)allocator.allocate(sizeof(brain__action__Led21On_GetResult_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_GetResult_Request));
  bool success = brain__action__Led21On_GetResult_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_GetResult_Request__destroy(brain__action__Led21On_GetResult_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_GetResult_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_GetResult_Request__Sequence__init(brain__action__Led21On_GetResult_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_GetResult_Request * data = NULL;

  if (size) {
    data = (brain__action__Led21On_GetResult_Request *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_GetResult_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_GetResult_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_GetResult_Request__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_GetResult_Request__Sequence__fini(brain__action__Led21On_GetResult_Request__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_GetResult_Request__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_GetResult_Request__Sequence *
brain__action__Led21On_GetResult_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_GetResult_Request__Sequence * array = (brain__action__Led21On_GetResult_Request__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_GetResult_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_GetResult_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_GetResult_Request__Sequence__destroy(brain__action__Led21On_GetResult_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_GetResult_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `result`
// already included above
// #include "brain/action/detail/led21_on__functions.h"

bool
brain__action__Led21On_GetResult_Response__init(brain__action__Led21On_GetResult_Response * msg)
{
  if (!msg) {
    return false;
  }
  // status
  // result
  if (!brain__action__Led21On_Result__init(&msg->result)) {
    brain__action__Led21On_GetResult_Response__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_GetResult_Response__fini(brain__action__Led21On_GetResult_Response * msg)
{
  if (!msg) {
    return;
  }
  // status
  // result
  brain__action__Led21On_Result__fini(&msg->result);
}

brain__action__Led21On_GetResult_Response *
brain__action__Led21On_GetResult_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_GetResult_Response * msg = (brain__action__Led21On_GetResult_Response *)allocator.allocate(sizeof(brain__action__Led21On_GetResult_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_GetResult_Response));
  bool success = brain__action__Led21On_GetResult_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_GetResult_Response__destroy(brain__action__Led21On_GetResult_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_GetResult_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_GetResult_Response__Sequence__init(brain__action__Led21On_GetResult_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_GetResult_Response * data = NULL;

  if (size) {
    data = (brain__action__Led21On_GetResult_Response *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_GetResult_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_GetResult_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_GetResult_Response__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_GetResult_Response__Sequence__fini(brain__action__Led21On_GetResult_Response__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_GetResult_Response__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_GetResult_Response__Sequence *
brain__action__Led21On_GetResult_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_GetResult_Response__Sequence * array = (brain__action__Led21On_GetResult_Response__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_GetResult_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_GetResult_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_GetResult_Response__Sequence__destroy(brain__action__Led21On_GetResult_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_GetResult_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__functions.h"
// Member `feedback`
// already included above
// #include "brain/action/detail/led21_on__functions.h"

bool
brain__action__Led21On_FeedbackMessage__init(brain__action__Led21On_FeedbackMessage * msg)
{
  if (!msg) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__init(&msg->goal_id)) {
    brain__action__Led21On_FeedbackMessage__fini(msg);
    return false;
  }
  // feedback
  if (!brain__action__Led21On_Feedback__init(&msg->feedback)) {
    brain__action__Led21On_FeedbackMessage__fini(msg);
    return false;
  }
  return true;
}

void
brain__action__Led21On_FeedbackMessage__fini(brain__action__Led21On_FeedbackMessage * msg)
{
  if (!msg) {
    return;
  }
  // goal_id
  unique_identifier_msgs__msg__UUID__fini(&msg->goal_id);
  // feedback
  brain__action__Led21On_Feedback__fini(&msg->feedback);
}

brain__action__Led21On_FeedbackMessage *
brain__action__Led21On_FeedbackMessage__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_FeedbackMessage * msg = (brain__action__Led21On_FeedbackMessage *)allocator.allocate(sizeof(brain__action__Led21On_FeedbackMessage), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(brain__action__Led21On_FeedbackMessage));
  bool success = brain__action__Led21On_FeedbackMessage__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
brain__action__Led21On_FeedbackMessage__destroy(brain__action__Led21On_FeedbackMessage * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    brain__action__Led21On_FeedbackMessage__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
brain__action__Led21On_FeedbackMessage__Sequence__init(brain__action__Led21On_FeedbackMessage__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_FeedbackMessage * data = NULL;

  if (size) {
    data = (brain__action__Led21On_FeedbackMessage *)allocator.zero_allocate(size, sizeof(brain__action__Led21On_FeedbackMessage), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = brain__action__Led21On_FeedbackMessage__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        brain__action__Led21On_FeedbackMessage__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
brain__action__Led21On_FeedbackMessage__Sequence__fini(brain__action__Led21On_FeedbackMessage__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      brain__action__Led21On_FeedbackMessage__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

brain__action__Led21On_FeedbackMessage__Sequence *
brain__action__Led21On_FeedbackMessage__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  brain__action__Led21On_FeedbackMessage__Sequence * array = (brain__action__Led21On_FeedbackMessage__Sequence *)allocator.allocate(sizeof(brain__action__Led21On_FeedbackMessage__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = brain__action__Led21On_FeedbackMessage__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
brain__action__Led21On_FeedbackMessage__Sequence__destroy(brain__action__Led21On_FeedbackMessage__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    brain__action__Led21On_FeedbackMessage__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}
