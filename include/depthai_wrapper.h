#pragma once
#include "rust/cxx.h"
#include <memory>

namespace dev
{
  namespace pnkv
  {

    struct DepthAISource;

    class DepthAIClient
    {
    public:
      DepthAIClient();
      ::std::int32_t next_frame(DepthAISource &src) const;

    private:
      class impl;
      std::shared_ptr<impl> impl;
      void push_frame(DepthAISource &src) const;
    };

    std::unique_ptr<DepthAIClient> new_depthai_client();

  }
}
