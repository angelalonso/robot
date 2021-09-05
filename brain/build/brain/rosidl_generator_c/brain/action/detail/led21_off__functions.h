// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from brain:action/Led21Off.idl
// generated code does not contain a copyright notice

#ifndef BRAIN__ACTION__DETAIL__LED21_OFF__FUNCTIONS_H_
#define BRAIN__ACTION__DETAIL__LED21_OFF__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "brain/msg/rosidl_generator_c__visibility_control.h"

#include "brain/action/detail/led21_off__struct.h"

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_Goal
 * )) before or use
 * brain__action__Led21Off_Goal__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_Goal__init(brain__action__Led21Off_Goal * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Goal__fini(brain__action__Led21Off_Goal * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_Goal__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_Goal *
brain__action__Led21Off_Goal__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_Goal__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Goal__destroy(brain__action__Led21Off_Goal * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_Goal__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_Goal__Sequence__init(brain__action__Led21Off_Goal__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_Goal__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Goal__Sequence__fini(brain__action__Led21Off_Goal__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_Goal__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_Goal__Sequence *
brain__action__Led21Off_Goal__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_Goal__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Goal__Sequence__destroy(brain__action__Led21Off_Goal__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_Result
 * )) before or use
 * brain__action__Led21Off_Result__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_Result__init(brain__action__Led21Off_Result * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Result__fini(brain__action__Led21Off_Result * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_Result__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_Result *
brain__action__Led21Off_Result__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_Result__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Result__destroy(brain__action__Led21Off_Result * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_Result__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_Result__Sequence__init(brain__action__Led21Off_Result__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_Result__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Result__Sequence__fini(brain__action__Led21Off_Result__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_Result__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_Result__Sequence *
brain__action__Led21Off_Result__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_Result__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Result__Sequence__destroy(brain__action__Led21Off_Result__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_Feedback
 * )) before or use
 * brain__action__Led21Off_Feedback__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_Feedback__init(brain__action__Led21Off_Feedback * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Feedback__fini(brain__action__Led21Off_Feedback * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_Feedback__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_Feedback *
brain__action__Led21Off_Feedback__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_Feedback__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Feedback__destroy(brain__action__Led21Off_Feedback * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_Feedback__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_Feedback__Sequence__init(brain__action__Led21Off_Feedback__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_Feedback__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Feedback__Sequence__fini(brain__action__Led21Off_Feedback__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_Feedback__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_Feedback__Sequence *
brain__action__Led21Off_Feedback__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_Feedback__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_Feedback__Sequence__destroy(brain__action__Led21Off_Feedback__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_SendGoal_Request
 * )) before or use
 * brain__action__Led21Off_SendGoal_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_SendGoal_Request__init(brain__action__Led21Off_SendGoal_Request * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Request__fini(brain__action__Led21Off_SendGoal_Request * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_SendGoal_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_SendGoal_Request *
brain__action__Led21Off_SendGoal_Request__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_SendGoal_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Request__destroy(brain__action__Led21Off_SendGoal_Request * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_SendGoal_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_SendGoal_Request__Sequence__init(brain__action__Led21Off_SendGoal_Request__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_SendGoal_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Request__Sequence__fini(brain__action__Led21Off_SendGoal_Request__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_SendGoal_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_SendGoal_Request__Sequence *
brain__action__Led21Off_SendGoal_Request__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_SendGoal_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Request__Sequence__destroy(brain__action__Led21Off_SendGoal_Request__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_SendGoal_Response
 * )) before or use
 * brain__action__Led21Off_SendGoal_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_SendGoal_Response__init(brain__action__Led21Off_SendGoal_Response * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Response__fini(brain__action__Led21Off_SendGoal_Response * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_SendGoal_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_SendGoal_Response *
brain__action__Led21Off_SendGoal_Response__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_SendGoal_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Response__destroy(brain__action__Led21Off_SendGoal_Response * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_SendGoal_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_SendGoal_Response__Sequence__init(brain__action__Led21Off_SendGoal_Response__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_SendGoal_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Response__Sequence__fini(brain__action__Led21Off_SendGoal_Response__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_SendGoal_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_SendGoal_Response__Sequence *
brain__action__Led21Off_SendGoal_Response__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_SendGoal_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_SendGoal_Response__Sequence__destroy(brain__action__Led21Off_SendGoal_Response__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_GetResult_Request
 * )) before or use
 * brain__action__Led21Off_GetResult_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_GetResult_Request__init(brain__action__Led21Off_GetResult_Request * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Request__fini(brain__action__Led21Off_GetResult_Request * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_GetResult_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_GetResult_Request *
brain__action__Led21Off_GetResult_Request__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_GetResult_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Request__destroy(brain__action__Led21Off_GetResult_Request * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_GetResult_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_GetResult_Request__Sequence__init(brain__action__Led21Off_GetResult_Request__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_GetResult_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Request__Sequence__fini(brain__action__Led21Off_GetResult_Request__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_GetResult_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_GetResult_Request__Sequence *
brain__action__Led21Off_GetResult_Request__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_GetResult_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Request__Sequence__destroy(brain__action__Led21Off_GetResult_Request__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_GetResult_Response
 * )) before or use
 * brain__action__Led21Off_GetResult_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_GetResult_Response__init(brain__action__Led21Off_GetResult_Response * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Response__fini(brain__action__Led21Off_GetResult_Response * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_GetResult_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_GetResult_Response *
brain__action__Led21Off_GetResult_Response__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_GetResult_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Response__destroy(brain__action__Led21Off_GetResult_Response * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_GetResult_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_GetResult_Response__Sequence__init(brain__action__Led21Off_GetResult_Response__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_GetResult_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Response__Sequence__fini(brain__action__Led21Off_GetResult_Response__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_GetResult_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_GetResult_Response__Sequence *
brain__action__Led21Off_GetResult_Response__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_GetResult_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_GetResult_Response__Sequence__destroy(brain__action__Led21Off_GetResult_Response__Sequence * array);

/// Initialize action/Led21Off message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * brain__action__Led21Off_FeedbackMessage
 * )) before or use
 * brain__action__Led21Off_FeedbackMessage__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_FeedbackMessage__init(brain__action__Led21Off_FeedbackMessage * msg);

/// Finalize action/Led21Off message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_FeedbackMessage__fini(brain__action__Led21Off_FeedbackMessage * msg);

/// Create action/Led21Off message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * brain__action__Led21Off_FeedbackMessage__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_FeedbackMessage *
brain__action__Led21Off_FeedbackMessage__create();

/// Destroy action/Led21Off message.
/**
 * It calls
 * brain__action__Led21Off_FeedbackMessage__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_FeedbackMessage__destroy(brain__action__Led21Off_FeedbackMessage * msg);


/// Initialize array of action/Led21Off messages.
/**
 * It allocates the memory for the number of elements and calls
 * brain__action__Led21Off_FeedbackMessage__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
bool
brain__action__Led21Off_FeedbackMessage__Sequence__init(brain__action__Led21Off_FeedbackMessage__Sequence * array, size_t size);

/// Finalize array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_FeedbackMessage__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_FeedbackMessage__Sequence__fini(brain__action__Led21Off_FeedbackMessage__Sequence * array);

/// Create array of action/Led21Off messages.
/**
 * It allocates the memory for the array and calls
 * brain__action__Led21Off_FeedbackMessage__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
brain__action__Led21Off_FeedbackMessage__Sequence *
brain__action__Led21Off_FeedbackMessage__Sequence__create(size_t size);

/// Destroy array of action/Led21Off messages.
/**
 * It calls
 * brain__action__Led21Off_FeedbackMessage__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_brain
void
brain__action__Led21Off_FeedbackMessage__Sequence__destroy(brain__action__Led21Off_FeedbackMessage__Sequence * array);

#ifdef __cplusplus
}
#endif

#endif  // BRAIN__ACTION__DETAIL__LED21_OFF__FUNCTIONS_H_
