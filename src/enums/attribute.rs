pub struct Attribute {
    kind: AttrKind,
    value: visa_sys::ViUInt16,
}

impl Attribute {
    pub fn new(kind: AttrKind, value: u16) -> Self {
        Self { kind, value }
    }
    pub fn kind(&self) -> AttrKind {
        self.kind
    }
    pub fn value(&self) -> u16 {
        self.value as _
    }
    pub fn set_value(self, value: u16) -> Self {
        Self {
            kind: self.kind,
            value,
        }
    }
}

pub use attributes::AttrKind;

mod attributes {
    #![allow(overflowing_literals)]
    impl AttrKind {
        pub fn value(self, v: u16) -> super::Attribute {
            super::Attribute {
                kind: self,
                value: v as _,
            }
        }
    }
    // todo: add description and range check
    consts_to_enum! {
        pub enum AttrKind: u32 {
            VI_ATTR_RSRC_CLASS	0xBFFF0001              //ViString N/A
            VI_ATTR_RSRC_NAME	0xBFFF0002              //ViRsrc N/A
            VI_ATTR_RSRC_IMPL_VERSION	0x3FFF0003              //ViVersion u32
            VI_ATTR_RSRC_LOCK_STATE	0x3FFF0004              //ViAccessMode [VI_NO_LOCK,VI_EXCLUSIVE_LOCK,VI_SHARED_LOCK]
            VI_ATTR_MAX_QUEUE_LENGTH	0x3FFF0005              //ViUInt32 NonZeroU32
            VI_ATTR_USER_DATA	0x3FFF0007              //ViAddr u32
            VI_ATTR_FDC_CHNL	0x3FFF000D              //ViUInt16 0-7
            VI_ATTR_FDC_MODE	0x3FFF000F              //
            VI_ATTR_FDC_GEN_SIGNAL_EN	0x3FFF0011              //
            VI_ATTR_FDC_USE_PAIR	0x3FFF0013              //
            VI_ATTR_SEND_END_EN	0x3FFF0016              //
            VI_ATTR_TERMCHAR	0x3FFF0018              //
            VI_ATTR_TMO_VALUE	0x3FFF001A              //
            VI_ATTR_GPIB_READDR_EN	0x3FFF001B              //
            VI_ATTR_IO_PROT	0x3FFF001C              //
            VI_ATTR_DMA_ALLOW_EN	0x3FFF001E              //
            VI_ATTR_ASRL_BAUD	0x3FFF0021              //
            VI_ATTR_ASRL_DATA_BITS	0x3FFF0022              //
            VI_ATTR_ASRL_PARITY	0x3FFF0023              //
            VI_ATTR_ASRL_STOP_BITS	0x3FFF0024              //
            VI_ATTR_ASRL_FLOW_CNTRL	0x3FFF0025              //
            VI_ATTR_RD_BUF_OPER_MODE	0x3FFF002A              //
            VI_ATTR_RD_BUF_SIZE	0x3FFF002B              //
            VI_ATTR_WR_BUF_OPER_MODE	0x3FFF002D              //
            VI_ATTR_WR_BUF_SIZE	0x3FFF002E              //
            VI_ATTR_SUPPRESS_END_EN	0x3FFF0036              //
            VI_ATTR_TERMCHAR_EN	0x3FFF0038              //
            VI_ATTR_DEST_ACCESS_PRIV	0x3FFF0039              //
            VI_ATTR_DEST_BYTE_ORDER	0x3FFF003A              //
            VI_ATTR_SRC_ACCESS_PRIV	0x3FFF003C              //
            VI_ATTR_SRC_BYTE_ORDER	0x3FFF003D              //
            VI_ATTR_SRC_INCREMENT	0x3FFF0040              //
            VI_ATTR_DEST_INCREMENT	0x3FFF0041              //
            VI_ATTR_WIN_ACCESS_PRIV	0x3FFF0045              //
            VI_ATTR_WIN_BYTE_ORDER	0x3FFF0047              //
            VI_ATTR_GPIB_ATN_STATE	0x3FFF0057              //
            VI_ATTR_GPIB_ADDR_STATE	0x3FFF005C              //
            VI_ATTR_GPIB_CIC_STATE	0x3FFF005E              //
            VI_ATTR_GPIB_NDAC_STATE	0x3FFF0062              //
            VI_ATTR_GPIB_SRQ_STATE	0x3FFF0067              //
            VI_ATTR_GPIB_SYS_CNTRL_STATE	0x3FFF0068              //
            VI_ATTR_GPIB_HS488_CBL_LEN	0x3FFF0069              //
            VI_ATTR_CMDR_LA	0x3FFF006B              //
            VI_ATTR_VXI_DEV_CLASS	0x3FFF006C              //
            VI_ATTR_MAINFRAME_LA	0x3FFF0070              //
            VI_ATTR_MANF_NAME	0xBFFF0072              //
            VI_ATTR_MODEL_NAME	0xBFFF0077              //
            VI_ATTR_VXI_VME_INTR_STATUS	0x3FFF008B              //
            VI_ATTR_VXI_TRIG_STATUS	0x3FFF008D              //
            VI_ATTR_VXI_VME_SYSFAIL_STATE	0x3FFF0094              //
            VI_ATTR_WIN_BASE_ADDR	0x3FFF0098              //
            VI_ATTR_WIN_SIZE	0x3FFF009A              //
            VI_ATTR_ASRL_AVAIL_NUM	0x3FFF00AC              //
            VI_ATTR_MEM_BASE	0x3FFF00AD              //
            VI_ATTR_ASRL_CTS_STATE	0x3FFF00AE              //
            VI_ATTR_ASRL_DCD_STATE	0x3FFF00AF              //
            VI_ATTR_ASRL_DSR_STATE	0x3FFF00B1              //
            VI_ATTR_ASRL_DTR_STATE	0x3FFF00B2              //
            VI_ATTR_ASRL_END_IN	0x3FFF00B3              //
            VI_ATTR_ASRL_END_OUT	0x3FFF00B4              //
            VI_ATTR_ASRL_REPLACE_CHAR	0x3FFF00BE              //
            VI_ATTR_ASRL_RI_STATE	0x3FFF00BF              //
            VI_ATTR_ASRL_RTS_STATE	0x3FFF00C0              //
            VI_ATTR_ASRL_XON_CHAR	0x3FFF00C1              //
            VI_ATTR_ASRL_XOFF_CHAR	0x3FFF00C2              //
            VI_ATTR_WIN_ACCESS	0x3FFF00C3              //
            VI_ATTR_RM_SESSION	0x3FFF00C4              //
            VI_ATTR_VXI_LA	0x3FFF00D5              //
            VI_ATTR_MANF_ID	0x3FFF00D9              //
            VI_ATTR_MEM_SIZE	0x3FFF00DD              //
            VI_ATTR_MEM_SPACE	0x3FFF00DE              //
            VI_ATTR_MODEL_CODE	0x3FFF00DF              //
            VI_ATTR_SLOT	0x3FFF00E8              //
            VI_ATTR_INTF_INST_NAME	0xBFFF00E9              //
            VI_ATTR_IMMEDIATE_SERV	0x3FFF0100              //
            VI_ATTR_INTF_PARENT_NUM	0x3FFF0101              //
            VI_ATTR_RSRC_SPEC_VERSION	0x3FFF0170              //
            VI_ATTR_INTF_TYPE	0x3FFF0171              //
            VI_ATTR_GPIB_PRIMARY_ADDR	0x3FFF0172              //
            VI_ATTR_GPIB_SECONDARY_ADDR	0x3FFF0173              //
            VI_ATTR_RSRC_MANF_NAME	0xBFFF0174              //
            VI_ATTR_RSRC_MANF_ID	0x3FFF0175              //
            VI_ATTR_INTF_NUM	0x3FFF0176              //
            VI_ATTR_TRIG_ID	0x3FFF0177              //
            VI_ATTR_GPIB_REN_STATE	0x3FFF0181              //
            VI_ATTR_GPIB_UNADDR_EN	0x3FFF0184              //
            VI_ATTR_DEV_STATUS_BYTE	0x3FFF0189              //
            VI_ATTR_FILE_APPEND_EN	0x3FFF0192              //
            VI_ATTR_VXI_TRIG_SUPPORT	0x3FFF0194              //
            VI_ATTR_TCPIP_ADDR	0xBFFF0195              //
            VI_ATTR_TCPIP_HOSTNAME	0xBFFF0196              //
            VI_ATTR_TCPIP_PORT	0x3FFF0197              //
            VI_ATTR_TCPIP_DEVICE_NAME	0xBFFF0199              //
            VI_ATTR_TCPIP_NODELAY	0x3FFF019A              //
            VI_ATTR_TCPIP_KEEPALIVE	0x3FFF019B              //
            VI_ATTR_4882_COMPLIANT	0x3FFF019F              //
            VI_ATTR_USB_SERIAL_NUM	0xBFFF01A0              //
            VI_ATTR_USB_INTFC_NUM	0x3FFF01A1              //
            VI_ATTR_USB_PROTOCOL	0x3FFF01A7              //
            VI_ATTR_USB_MAX_INTR_SIZE	0x3FFF01AF              //
            VI_ATTR_JOB_ID	0x3FFF4006              //
            VI_ATTR_EVENT_TYPE	0x3FFF4010              //
            VI_ATTR_SIGP_STATUS_ID	0x3FFF4011              //
            VI_ATTR_RECV_TRIG_ID	0x3FFF4012              //
            VI_ATTR_INTR_STATUS_ID	0x3FFF4023              //
            VI_ATTR_STATUS	0x3FFF4025              //
            VI_ATTR_RET_COUNT	0x3FFF4026              //
            VI_ATTR_BUFFER	0x3FFF4027              //
            VI_ATTR_RECV_INTR_LEVEL	0x3FFF4041              //
            VI_ATTR_OPER_NAME	0xBFFF4042              //
            VI_ATTR_GPIB_RECV_CIC_STATE	0x3FFF4193              //
            VI_ATTR_RECV_TCPIP_ADDR	0xBFFF4198              //
            VI_ATTR_USB_RECV_INTR_SIZE	0x3FFF41B0              //
            VI_ATTR_USB_RECV_INTR_DATA	0xBFFF41B1              //
            VI_ATTR_PXI_DEV_NUM	0x3FFF0201              //
            VI_ATTR_PXI_FUNC_NUM	0x3FFF0202              //
            VI_ATTR_PXI_BUS_NUM	0x3FFF0205              //
            VI_ATTR_PXI_CHASSIS	0x3FFF0206              //
            VI_ATTR_PXI_SLOTPATH	0xBFFF0207              //
            VI_ATTR_PXI_SLOT_LBUS_LEFT	0x3FFF0208              //
            VI_ATTR_PXI_SLOT_LBUS_RIGHT	0x3FFF0209              //
            VI_ATTR_PXI_TRIG_BUS	0x3FFF020A              //
            VI_ATTR_PXI_STAR_TRIG_BUS	0x3FFF020B              //
            VI_ATTR_PXI_STAR_TRIG_LINE	0x3FFF020C              //
            VI_ATTR_PXI_MEM_TYPE_BAR0	0x3FFF0211              //
            VI_ATTR_PXI_MEM_TYPE_BAR1	0x3FFF0212              //
            VI_ATTR_PXI_MEM_TYPE_BAR2	0x3FFF0213              //
            VI_ATTR_PXI_MEM_TYPE_BAR3	0x3FFF0214              //
            VI_ATTR_PXI_MEM_TYPE_BAR4	0x3FFF0215              //
            VI_ATTR_PXI_MEM_TYPE_BAR5	0x3FFF0216              //
            VI_ATTR_PXI_MEM_BASE_BAR0_32	0x3FFF0221              //
            VI_ATTR_PXI_MEM_BASE_BAR1_32	0x3FFF0222              //
            VI_ATTR_PXI_MEM_BASE_BAR2_32	0x3FFF0223              //
            VI_ATTR_PXI_MEM_BASE_BAR3_32	0x3FFF0224              //
            VI_ATTR_PXI_MEM_BASE_BAR4_32	0x3FFF0225              //
            VI_ATTR_PXI_MEM_BASE_BAR5_32	0x3FFF0226              //
            VI_ATTR_PXI_MEM_SIZE_BAR0_32	0x3FFF0231              //
            VI_ATTR_PXI_MEM_SIZE_BAR1_32	0x3FFF0232              //
            VI_ATTR_PXI_MEM_SIZE_BAR2_32	0x3FFF0233              //
            VI_ATTR_PXI_MEM_SIZE_BAR3_32	0x3FFF0234              //
            VI_ATTR_PXI_MEM_SIZE_BAR4_32	0x3FFF0235              //
            VI_ATTR_PXI_MEM_SIZE_BAR5_32	0x3FFF0236              //
            VI_ATTR_PXI_IS_EXPRESS	0x3FFF0240              //
            VI_ATTR_PXI_SLOT_LWIDTH	0x3FFF0241              //
            VI_ATTR_PXI_MAX_LWIDTH	0x3FFF0242              //
            VI_ATTR_PXI_ACTUAL_LWIDTH	0x3FFF0243              //
            VI_ATTR_PXI_DSTAR_BUS	0x3FFF0244              //
            VI_ATTR_PXI_DSTAR_SET	0x3FFF0245              //
            VI_ATTR_TCPIP_SERVER_CERT_ISSUER_NAME	0xBFFF0270              //
            VI_ATTR_TCPIP_SERVER_CERT_SUBJECT_NAME	0xBFFF0271              //
            VI_ATTR_TCPIP_SERVER_CERT_EXPIRATION_DATE	0xBFFF0272              //
            VI_ATTR_TCPIP_SERVER_CERT_IS_PERPETUAL	0x3FFF0273              //
            VI_ATTR_TCPIP_SASL_MECHANISM	0xBFFF0274              //
            VI_ATTR_TCPIP_TLS_CIPHER_SUITE	0xBFFF0275              //
            VI_ATTR_TCPIP_HISLIP_OVERLAP_EN	0x3FFF0300              //
            VI_ATTR_TCPIP_HISLIP_VERSION	0x3FFF0301              //
            VI_ATTR_TCPIP_HISLIP_MAX_MESSAGE_KB	0x3FFF0302              //
            VI_ATTR_TCPIP_IS_HISLIP	0x3FFF0303              //
            VI_ATTR_TCPIP_HISLIP_ENCRYPTION_EN	0x3FFF0304              //
            VI_ATTR_PXI_RECV_INTR_SEQ	0x3FFF4240              //
            VI_ATTR_PXI_RECV_INTR_DATA	0x3FFF4241              //
            VI_ATTR_PXI_SRC_TRIG_BUS	0x3FFF020D              //
            VI_ATTR_PXI_DEST_TRIG_BUS	0x3FFF020E              //
            VI_ATTR_PXI_MEM_BASE_BAR0_64	0x3FFF0228              //
            VI_ATTR_PXI_MEM_BASE_BAR1_64	0x3FFF0229              //
            VI_ATTR_PXI_MEM_BASE_BAR2_64	0x3FFF022A              //
            VI_ATTR_PXI_MEM_BASE_BAR3_64	0x3FFF022B              //
            VI_ATTR_PXI_MEM_BASE_BAR4_64	0x3FFF022C              //
            VI_ATTR_PXI_MEM_BASE_BAR5_64	0x3FFF022D              //
            VI_ATTR_PXI_MEM_SIZE_BAR0_64	0x3FFF0238              //
            VI_ATTR_PXI_MEM_SIZE_BAR1_64	0x3FFF0239              //
            VI_ATTR_PXI_MEM_SIZE_BAR2_64	0x3FFF023A              //
            VI_ATTR_PXI_MEM_SIZE_BAR3_64	0x3FFF023B              //
            VI_ATTR_PXI_MEM_SIZE_BAR4_64	0x3FFF023C              //
            VI_ATTR_PXI_MEM_SIZE_BAR5_64	0x3FFF023D              //
            VI_ATTR_PXI_ALLOW_WRITE_COMBINE	0x3FFF0246              //
        }
    }
}
