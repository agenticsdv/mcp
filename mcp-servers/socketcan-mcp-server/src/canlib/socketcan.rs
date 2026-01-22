#![allow(dead_code)]

use anyhow::Result;

use rmcp::{
    ServerHandler,ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
//use socketcan::{CanFrame, CanSocket, Frame, StandardId};
use socketcan::{CANSocket, CANFrame};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CANFrame {
    #[schemars(description = "the left hand side number")]
    pub id: u32, //canid_t,
    pub dlc: u8,
    payload: [u8; 8],
}

#[derive(Debug, Clone)]
pub struct CANServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CANServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    #[tool(description = "Send a CAN frame")]
    fn send(&self, Parameters(CANFrame { id, dlc, payload }): Parameters<CANFrame>) -> Result<String, Box<dyn std::error::Error>> {
        let socket = CanSocket::open("vcan0")?;
        let mut iface = SocketCanIface { socket };
        let frame = CanFrame::new(id,&payload[..dlc as usize])?;)
        socket.write_frame(frame);
        //let frame = CanFrame::new(sid.into(), &payload).unwrap(); 
        //let frame = CanFrame::new(StandardId::new(id).unwrap(), &payload[..dlc as usize]).unwrap();
        /*let cframe = can_frame {
            can_id : id,
            can_dlc: dlc,
            data: payload,
        };
        //let frame = CanDataFrame::from(cframe);*/
        //let sid = StandardId::new(id).unwrap();
        //iface.transmit(&frame)?;
        Ok(format!("CAN Frame ID: {}, DLC: {}, Data: {:?}", id, dlc, payload))
    }
}

#[tool_handler]
impl ServerHandler for CANServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple calculator".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
