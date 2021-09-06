// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from brain:action/Led21.idl
// generated code does not contain a copyright notice

#ifndef BRAIN__ACTION__DETAIL__LED21__TRAITS_HPP_
#define BRAIN__ACTION__DETAIL__LED21__TRAITS_HPP_

#include "brain/action/detail/led21__struct.hpp"
#include <stdint.h>
#include <rosidl_runtime_cpp/traits.hpp>
#include <sstream>
#include <string>
#include <type_traits>

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: turn_on
  {
    out << "turn_on: ";
    rosidl_generator_traits::value_to_yaml(msg.turn_on, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: turn_on
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "turn_on: ";
    rosidl_generator_traits::value_to_yaml(msg.turn_on, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_Goal & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_Goal>()
{
  return "brain::action::Led21_Goal";
}

template<>
inline const char * name<brain::action::Led21_Goal>()
{
  return "brain/action/Led21_Goal";
}

template<>
struct has_fixed_size<brain::action::Led21_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<brain::action::Led21_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<brain::action::Led21_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_Result & msg,
  std::ostream & out)
{
  out << "{";
  // member: confirmed
  {
    out << "confirmed: ";
    rosidl_generator_traits::value_to_yaml(msg.confirmed, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: confirmed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "confirmed: ";
    rosidl_generator_traits::value_to_yaml(msg.confirmed, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_Result & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_Result>()
{
  return "brain::action::Led21_Result";
}

template<>
inline const char * name<brain::action::Led21_Result>()
{
  return "brain/action/Led21_Result";
}

template<>
struct has_fixed_size<brain::action::Led21_Result>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<brain::action::Led21_Result>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<brain::action::Led21_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: process_feed
  {
    out << "process_feed: ";
    rosidl_generator_traits::value_to_yaml(msg.process_feed, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: process_feed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "process_feed: ";
    rosidl_generator_traits::value_to_yaml(msg.process_feed, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_Feedback & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_Feedback>()
{
  return "brain::action::Led21_Feedback";
}

template<>
inline const char * name<brain::action::Led21_Feedback>()
{
  return "brain/action/Led21_Feedback";
}

template<>
struct has_fixed_size<brain::action::Led21_Feedback>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<brain::action::Led21_Feedback>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<brain::action::Led21_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "brain/action/detail/led21__traits.hpp"

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_SendGoal_Request & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_SendGoal_Request>()
{
  return "brain::action::Led21_SendGoal_Request";
}

template<>
inline const char * name<brain::action::Led21_SendGoal_Request>()
{
  return "brain/action/Led21_SendGoal_Request";
}

template<>
struct has_fixed_size<brain::action::Led21_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<brain::action::Led21_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<brain::action::Led21_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<brain::action::Led21_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<brain::action::Led21_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_SendGoal_Response & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_SendGoal_Response>()
{
  return "brain::action::Led21_SendGoal_Response";
}

template<>
inline const char * name<brain::action::Led21_SendGoal_Response>()
{
  return "brain/action/Led21_SendGoal_Response";
}

template<>
struct has_fixed_size<brain::action::Led21_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<brain::action::Led21_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<brain::action::Led21_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<brain::action::Led21_SendGoal>()
{
  return "brain::action::Led21_SendGoal";
}

template<>
inline const char * name<brain::action::Led21_SendGoal>()
{
  return "brain/action/Led21_SendGoal";
}

template<>
struct has_fixed_size<brain::action::Led21_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<brain::action::Led21_SendGoal_Request>::value &&
    has_fixed_size<brain::action::Led21_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<brain::action::Led21_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<brain::action::Led21_SendGoal_Request>::value &&
    has_bounded_size<brain::action::Led21_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<brain::action::Led21_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<brain::action::Led21_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<brain::action::Led21_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_GetResult_Request & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_GetResult_Request>()
{
  return "brain::action::Led21_GetResult_Request";
}

template<>
inline const char * name<brain::action::Led21_GetResult_Request>()
{
  return "brain/action/Led21_GetResult_Request";
}

template<>
struct has_fixed_size<brain::action::Led21_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<brain::action::Led21_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<brain::action::Led21_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "brain/action/detail/led21__traits.hpp"

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_GetResult_Response & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_GetResult_Response>()
{
  return "brain::action::Led21_GetResult_Response";
}

template<>
inline const char * name<brain::action::Led21_GetResult_Response>()
{
  return "brain/action/Led21_GetResult_Response";
}

template<>
struct has_fixed_size<brain::action::Led21_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<brain::action::Led21_Result>::value> {};

template<>
struct has_bounded_size<brain::action::Led21_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<brain::action::Led21_Result>::value> {};

template<>
struct is_message<brain::action::Led21_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<brain::action::Led21_GetResult>()
{
  return "brain::action::Led21_GetResult";
}

template<>
inline const char * name<brain::action::Led21_GetResult>()
{
  return "brain/action/Led21_GetResult";
}

template<>
struct has_fixed_size<brain::action::Led21_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<brain::action::Led21_GetResult_Request>::value &&
    has_fixed_size<brain::action::Led21_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<brain::action::Led21_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<brain::action::Led21_GetResult_Request>::value &&
    has_bounded_size<brain::action::Led21_GetResult_Response>::value
  >
{
};

template<>
struct is_service<brain::action::Led21_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<brain::action::Led21_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<brain::action::Led21_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "brain/action/detail/led21__traits.hpp"

namespace brain
{

namespace action
{

inline void to_flow_style_yaml(
  const Led21_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Led21_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Led21_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace brain

namespace rosidl_generator_traits
{

[[deprecated("use brain::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const brain::action::Led21_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  brain::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use brain::action::to_yaml() instead")]]
inline std::string to_yaml(const brain::action::Led21_FeedbackMessage & msg)
{
  return brain::action::to_yaml(msg);
}

template<>
inline const char * data_type<brain::action::Led21_FeedbackMessage>()
{
  return "brain::action::Led21_FeedbackMessage";
}

template<>
inline const char * name<brain::action::Led21_FeedbackMessage>()
{
  return "brain/action/Led21_FeedbackMessage";
}

template<>
struct has_fixed_size<brain::action::Led21_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<brain::action::Led21_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<brain::action::Led21_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<brain::action::Led21_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<brain::action::Led21_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<brain::action::Led21>
  : std::true_type
{
};

template<>
struct is_action_goal<brain::action::Led21_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<brain::action::Led21_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<brain::action::Led21_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // BRAIN__ACTION__DETAIL__LED21__TRAITS_HPP_
