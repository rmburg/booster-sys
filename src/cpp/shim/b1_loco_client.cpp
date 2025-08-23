#include <booster/robot/b1/b1_loco_client.hpp>
#include <memory>

namespace booster {
namespace robot {
namespace b1 {

std::unique_ptr<B1LocoClient> b1_loco_client_new() {
  return std::make_unique<B1LocoClient>();
}

} // namespace b1
} // namespace robot
} // namespace booster
