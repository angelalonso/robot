// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from action_interfaces:action/MotorR.idl
// generated code does not contain a copyright notice

#ifndef ACTION_INTERFACES__ACTION__DETAIL__MOTOR_R__BUILDER_HPP_
#define ACTION_INTERFACES__ACTION__DETAIL__MOTOR_R__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "action_interfaces/action/detail/motor_r__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_Goal_speed
{
public:
  Init_MotorR_Goal_speed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_interfaces::action::MotorR_Goal speed(::action_interfaces::action::MotorR_Goal::_speed_type arg)
  {
    msg_.speed = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_Goal>()
{
  return action_interfaces::action::builder::Init_MotorR_Goal_speed();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_Result_confirmed
{
public:
  Init_MotorR_Result_confirmed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_interfaces::action::MotorR_Result confirmed(::action_interfaces::action::MotorR_Result::_confirmed_type arg)
  {
    msg_.confirmed = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_Result>()
{
  return action_interfaces::action::builder::Init_MotorR_Result_confirmed();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_Feedback_process_feed
{
public:
  Init_MotorR_Feedback_process_feed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_interfaces::action::MotorR_Feedback process_feed(::action_interfaces::action::MotorR_Feedback::_process_feed_type arg)
  {
    msg_.process_feed = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_Feedback>()
{
  return action_interfaces::action::builder::Init_MotorR_Feedback_process_feed();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_SendGoal_Request_goal
{
public:
  explicit Init_MotorR_SendGoal_Request_goal(::action_interfaces::action::MotorR_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::action_interfaces::action::MotorR_SendGoal_Request goal(::action_interfaces::action::MotorR_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_SendGoal_Request msg_;
};

class Init_MotorR_SendGoal_Request_goal_id
{
public:
  Init_MotorR_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MotorR_SendGoal_Request_goal goal_id(::action_interfaces::action::MotorR_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_MotorR_SendGoal_Request_goal(msg_);
  }

private:
  ::action_interfaces::action::MotorR_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_SendGoal_Request>()
{
  return action_interfaces::action::builder::Init_MotorR_SendGoal_Request_goal_id();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_SendGoal_Response_stamp
{
public:
  explicit Init_MotorR_SendGoal_Response_stamp(::action_interfaces::action::MotorR_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::action_interfaces::action::MotorR_SendGoal_Response stamp(::action_interfaces::action::MotorR_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_SendGoal_Response msg_;
};

class Init_MotorR_SendGoal_Response_accepted
{
public:
  Init_MotorR_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MotorR_SendGoal_Response_stamp accepted(::action_interfaces::action::MotorR_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_MotorR_SendGoal_Response_stamp(msg_);
  }

private:
  ::action_interfaces::action::MotorR_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_SendGoal_Response>()
{
  return action_interfaces::action::builder::Init_MotorR_SendGoal_Response_accepted();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_GetResult_Request_goal_id
{
public:
  Init_MotorR_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_interfaces::action::MotorR_GetResult_Request goal_id(::action_interfaces::action::MotorR_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_GetResult_Request>()
{
  return action_interfaces::action::builder::Init_MotorR_GetResult_Request_goal_id();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_GetResult_Response_result
{
public:
  explicit Init_MotorR_GetResult_Response_result(::action_interfaces::action::MotorR_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::action_interfaces::action::MotorR_GetResult_Response result(::action_interfaces::action::MotorR_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_GetResult_Response msg_;
};

class Init_MotorR_GetResult_Response_status
{
public:
  Init_MotorR_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MotorR_GetResult_Response_result status(::action_interfaces::action::MotorR_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_MotorR_GetResult_Response_result(msg_);
  }

private:
  ::action_interfaces::action::MotorR_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_GetResult_Response>()
{
  return action_interfaces::action::builder::Init_MotorR_GetResult_Response_status();
}

}  // namespace action_interfaces


namespace action_interfaces
{

namespace action
{

namespace builder
{

class Init_MotorR_FeedbackMessage_feedback
{
public:
  explicit Init_MotorR_FeedbackMessage_feedback(::action_interfaces::action::MotorR_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::action_interfaces::action::MotorR_FeedbackMessage feedback(::action_interfaces::action::MotorR_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_interfaces::action::MotorR_FeedbackMessage msg_;
};

class Init_MotorR_FeedbackMessage_goal_id
{
public:
  Init_MotorR_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MotorR_FeedbackMessage_feedback goal_id(::action_interfaces::action::MotorR_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_MotorR_FeedbackMessage_feedback(msg_);
  }

private:
  ::action_interfaces::action::MotorR_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_interfaces::action::MotorR_FeedbackMessage>()
{
  return action_interfaces::action::builder::Init_MotorR_FeedbackMessage_goal_id();
}

}  // namespace action_interfaces

#endif  // ACTION_INTERFACES__ACTION__DETAIL__MOTOR_R__BUILDER_HPP_
