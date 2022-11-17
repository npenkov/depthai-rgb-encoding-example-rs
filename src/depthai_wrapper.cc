#include "include/depthai_wrapper.h"
#include "depthai-rust/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <atomic>
#include <thread>
#include <chrono>
#include <memory>
#include <iostream>
#include <unordered_map>
#include "depthai/depthai.hpp"

namespace dev
{
  namespace pnkv
  {

    // Implement simple RawData wrapper for Rust
    class DepthAIClient::impl
    {
      friend DepthAIClient;

    private:
      std::shared_ptr<dai::DataOutputQueue> outputQueue;
      std::shared_ptr<dai::Device> device;
    };

    DepthAIClient::DepthAIClient() : impl(new class DepthAIClient::impl)
    {
      // Create pipeline
      auto pipeline = std::make_shared<dai::Pipeline>();

      // Define sources and outputs
      auto camRgb = pipeline->create<dai::node::ColorCamera>();
      auto videoEnc = pipeline->create<dai::node::VideoEncoder>();
      auto xout = pipeline->create<dai::node::XLinkOut>();

      xout->setStreamName("camRgb");

      // Properties
      camRgb->setBoardSocket(dai::CameraBoardSocket::RGB);
      camRgb->setResolution(dai::ColorCameraProperties::SensorResolution::THE_720_P);
      videoEnc->setDefaultProfilePreset(30, dai::VideoEncoderProperties::Profile::H264_BASELINE);

      // Linking
      camRgb->video.link(videoEnc->input);
      videoEnc->bitstream.link(xout->input);

      auto deviceInfoVec = dai::Device::getAllAvailableDevices();
      const auto usbSpeed = dai::UsbSpeed::SUPER;
      auto openVinoVersion = dai::OpenVINO::Version::VERSION_2021_4;

      std::map<std::string, std::shared_ptr<dai::DataOutputQueue>> qRgbMap;
      std::vector<std::shared_ptr<dai::Device>> devices;

      // Get the first device
      for (auto &deviceInfo : deviceInfoVec)
      {
        auto device = std::make_shared<dai::Device>(openVinoVersion, deviceInfo, usbSpeed);
        device->startPipeline(*pipeline);
        impl->device = device;
        auto outputQueue = impl->device->getOutputQueue("camRgb", 30, true);
        impl->outputQueue = outputQueue;
        break;
      }
    }

    // Start fetching encoded frames and pushing them to DepthAISource
    ::std::int32_t DepthAIClient::next_frame(DepthAISource &src) const
    {
      push_frame(src);
      return 0;
    }

    void DepthAIClient::push_frame(DepthAISource &src) const
    {
      // Output queue will be used to get the encoded data from the output defined above
      auto h265Packet = impl->outputQueue->get<dai::ImgFrame>();
      ::rust::Slice<const ::std::uint8_t> data = ::rust::Slice<const ::std::uint8_t>(h265Packet->getData().data(), h265Packet->getData().size());
      post_frame(src, data, h265Packet->getData().size());
      return;
    }

    std::unique_ptr<DepthAIClient> new_depthai_client()
    {
      return std::make_unique<DepthAIClient>();
    }

  }
}
