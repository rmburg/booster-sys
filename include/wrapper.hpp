#include <memory>
#include "booster/robot/channel/channel_factory.hpp"

// See https://github.com/dtolnay/cxx/issues/280#issuecomment-1344153115
template <typename T, typename... Args>
std::unique_ptr<T> construct_unique(Args... args) {
  return std::unique_ptr<T>(new T(args...));
}

inline void init_channel_factory(const std::string &network_interface) {
  booster::robot::ChannelFactory::Instance()->Init(0, network_interface);
}
