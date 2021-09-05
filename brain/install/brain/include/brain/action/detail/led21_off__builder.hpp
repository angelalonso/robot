// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from brain:action/Led21Off.idl
// generated code does not contain a copyright notice

#ifndef BRAIN__ACTION__DETAIL__LED21_OFF__BUILDER_HPP_
#define BRAIN__ACTION__DETAIL__LED21_OFF__BUILDER_HPP_

#include "brain/action/detail/led21_off__struct.hpp"
#include <rosidl_runtime_cpp/message_initialization.hpp>
#include <algorithm>
#include <utility>


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_Goal_turn_off
{
public:
  Init_Led21Off_Goal_turn_off()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::brain::action::Led21Off_Goal turn_off(::brain::action::Led21Off_Goal::_turn_off_type arg)
  {
    msg_.turn_off = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_Goal>()
{
  return brain::action::builder::Init_Led21Off_Goal_turn_off();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_Result_confirmed
{
public:
  Init_Led21Off_Result_confirmed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::brain::action::Led21Off_Result confirmed(::brain::action::Led21Off_Result::_confirmed_type arg)
  {
    msg_.confirmed = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_Result>()
{
  return brain::action::builder::Init_Led21Off_Result_confirmed();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_Feedback_process_feed
{
public:
  Init_Led21Off_Feedback_process_feed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::brain::action::Led21Off_Feedback process_feed(::brain::action::Led21Off_Feedback::_process_feed_type arg)
  {
    msg_.process_feed = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_Feedback>()
{
  return brain::action::builder::Init_Led21Off_Feedback_process_feed();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_SendGoal_Request_goal
{
public:
  explicit Init_Led21Off_SendGoal_Request_goal(::brain::action::Led21Off_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::brain::action::Led21Off_SendGoal_Request goal(::brain::action::Led21Off_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_SendGoal_Request msg_;
};

class Init_Led21Off_SendGoal_Request_goal_id
{
public:
  Init_Led21Off_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Led21Off_SendGoal_Request_goal goal_id(::brain::action::Led21Off_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_Led21Off_SendGoal_Request_goal(msg_);
  }

private:
  ::brain::action::Led21Off_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_SendGoal_Request>()
{
  return brain::action::builder::Init_Led21Off_SendGoal_Request_goal_id();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_SendGoal_Response_stamp
{
public:
  explicit Init_Led21Off_SendGoal_Response_stamp(::brain::action::Led21Off_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::brain::action::Led21Off_SendGoal_Response stamp(::brain::action::Led21Off_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_SendGoal_Response msg_;
};

class Init_Led21Off_SendGoal_Response_accepted
{
public:
  Init_Led21Off_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Led21Off_SendGoal_Response_stamp accepted(::brain::action::Led21Off_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_Led21Off_SendGoal_Response_stamp(msg_);
  }

private:
  ::brain::action::Led21Off_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_SendGoal_Response>()
{
  return brain::action::builder::Init_Led21Off_SendGoal_Response_accepted();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_GetResult_Request_goal_id
{
public:
  Init_Led21Off_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::brain::action::Led21Off_GetResult_Request goal_id(::brain::action::Led21Off_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_GetResult_Request>()
{
  return brain::action::builder::Init_Led21Off_GetResult_Request_goal_id();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_GetResult_Response_result
{
public:
  explicit Init_Led21Off_GetResult_Response_result(::brain::action::Led21Off_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::brain::action::Led21Off_GetResult_Response result(::brain::action::Led21Off_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_GetResult_Response msg_;
};

class Init_Led21Off_GetResult_Response_status
{
public:
  Init_Led21Off_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Led21Off_GetResult_Response_result status(::brain::action::Led21Off_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_Led21Off_GetResult_Response_result(msg_);
  }

private:
  ::brain::action::Led21Off_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_GetResult_Response>()
{
  return brain::action::builder::Init_Led21Off_GetResult_Response_status();
}

}  // namespace brain


namespace brain
{

namespace action
{

namespace builder
{

class Init_Led21Off_FeedbackMessage_feedback
{
public:
  explicit Init_Led21Off_FeedbackMessage_feedback(::brain::action::Led21Off_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::brain::action::Led21Off_FeedbackMessage feedback(::brain::action::Led21Off_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::brain::action::Led21Off_FeedbackMessage msg_;
};

class Init_Led21Off_FeedbackMessage_goal_id
{
public:
  Init_Led21Off_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Led21Off_FeedbackMessage_feedback goal_id(::brain::action::Led21Off_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_Led21Off_FeedbackMessage_feedback(msg_);
  }

private:
  ::brain::action::Led21Off_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::brain::action::Led21Off_FeedbackMessage>()
{
  return brain::action::builder::Init_Led21Off_FeedbackMessage_goal_id();
}

}  // namespace brain

#endif  // BRAIN__ACTION__DETAIL__LED21_OFF__BUILDER_HPP_
