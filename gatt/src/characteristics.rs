use crate::Uuid;

/// Device Name
pub const DEVICE_NAME: Uuid = Uuid::new_uuid16(0x2A00);
/// Appearance
pub const APPEARANCE: Uuid = Uuid::new_uuid16(0x2A01);
/// Peripheral Privacy Flag
pub const PERIPHERAL_PRIVACY_FLAG: Uuid = Uuid::new_uuid16(0x2A02);
/// Reconnection Address
pub const RECONNECTION_ADDRESS: Uuid = Uuid::new_uuid16(0x2A03);
/// Peripheral Preferred Connection Parameters
pub const PERIPHERAL_PREFERRED_CONNECTION_PARAMETERS: Uuid = Uuid::new_uuid16(0x2A04);
/// Service Changed
pub const SERVICE_CHANGED: Uuid = Uuid::new_uuid16(0x2A05);
/// Alert Level
pub const ALERT_LEVEL: Uuid = Uuid::new_uuid16(0x2A06);
/// Tx Power Level
pub const TX_POWER_LEVEL: Uuid = Uuid::new_uuid16(0x2A07);
/// Date Time
pub const DATE_TIME: Uuid = Uuid::new_uuid16(0x2A08);
/// Day of Week
pub const DAY_OF_WEEK: Uuid = Uuid::new_uuid16(0x2A09);
/// Day Date Time
pub const DAY_DATE_TIME: Uuid = Uuid::new_uuid16(0x2A0A);
/// Exact Time 256
pub const EXACT_TIME_256: Uuid = Uuid::new_uuid16(0x2A0C);
/// DST Offset
pub const DST_OFFSET: Uuid = Uuid::new_uuid16(0x2A0D);
/// Time Zone
pub const TIME_ZONE: Uuid = Uuid::new_uuid16(0x2A0E);
/// Local Time Information
pub const LOCAL_TIME_INFORMATION: Uuid = Uuid::new_uuid16(0x2A0F);
/// Time with DST
pub const TIME_WITH_DST: Uuid = Uuid::new_uuid16(0x2A11);
/// Time Accuracy
pub const TIME_ACCURACY: Uuid = Uuid::new_uuid16(0x2A12);
/// Time Source
pub const TIME_SOURCE: Uuid = Uuid::new_uuid16(0x2A13);
/// Reference Time Information
pub const REFERENCE_TIME_INFORMATION: Uuid = Uuid::new_uuid16(0x2A14);
/// Time Update Control Point
pub const TIME_UPDATE_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A16);
/// Time Update State
pub const TIME_UPDATE_STATE: Uuid = Uuid::new_uuid16(0x2A17);
/// Glucose Measurement
pub const GLUCOSE_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A18);
/// Battery Level
pub const BATTERY_LEVEL: Uuid = Uuid::new_uuid16(0x2A19);
/// Temperature Measurement
pub const TEMPERATURE_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A1C);
/// Temperature Type
pub const TEMPERATURE_TYPE: Uuid = Uuid::new_uuid16(0x2A1D);
/// Intermediate Temperature
pub const INTERMEDIATE_TEMPERATURE: Uuid = Uuid::new_uuid16(0x2A1E);
/// Measurement Interval
pub const MEASUREMENT_INTERVAL: Uuid = Uuid::new_uuid16(0x2A21);
/// Boot Keyboard Input Report
pub const BOOT_KEYBOARD_INPUT_REPORT: Uuid = Uuid::new_uuid16(0x2A22);
/// System ID
pub const SYSTEM_ID: Uuid = Uuid::new_uuid16(0x2A23);
/// Model Number String
pub const MODEL_NUMBER_STRING: Uuid = Uuid::new_uuid16(0x2A24);
/// Serial Number String
pub const SERIAL_NUMBER_STRING: Uuid = Uuid::new_uuid16(0x2A25);
/// Firmware Revision String
pub const FIRMWARE_REVISION_STRING: Uuid = Uuid::new_uuid16(0x2A26);
/// Hardware Revision String
pub const HARDWARE_REVISION_STRING: Uuid = Uuid::new_uuid16(0x2A27);
/// Software Revision String
pub const SOFTWARE_REVISION_STRING: Uuid = Uuid::new_uuid16(0x2A28);
/// Manufacturer Name String
pub const MANUFACTURER_NAME_STRING: Uuid = Uuid::new_uuid16(0x2A29);
/// IEEE 11073-20601 Regulatory Certification Data List
pub const IEEE_11073_20601_REGULATORY_CERTIFICATION_DATA_LIST: Uuid = Uuid::new_uuid16(0x2A2A);
/// Current Time
pub const CURRENT_TIME: Uuid = Uuid::new_uuid16(0x2A2B);
/// Scan Refresh
pub const SCAN_REFRESH: Uuid = Uuid::new_uuid16(0x2A31);
/// Boot Keyboard Output Report
pub const BOOT_KEYBOARD_OUTPUT_REPORT: Uuid = Uuid::new_uuid16(0x2A32);
/// Boot Mouse Input Report
pub const BOOT_MOUSE_INPUT_REPORT: Uuid = Uuid::new_uuid16(0x2A33);
/// Glucose Measurement Context
pub const GLUCOSE_MEASUREMENT_CONTEXT: Uuid = Uuid::new_uuid16(0x2A34);
/// Blood Pressure Measurement
pub const BLOOD_PRESSURE_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A35);
/// Intermediate Cuff Pressure
pub const INTERMEDIATE_CUFF_PRESSURE: Uuid = Uuid::new_uuid16(0x2A36);
/// Heart Rate Measurement
pub const HEART_RATE_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A37);
/// Body Sensor Location
pub const BODY_SENSOR_LOCATION: Uuid = Uuid::new_uuid16(0x2A38);
/// Heart Rate Control Point
pub const HEART_RATE_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A39);
/// Alert Status
pub const ALERT_STATUS: Uuid = Uuid::new_uuid16(0x2A3F);
/// Ringer Control Point
pub const RINGER_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A40);
/// Ringer Setting
pub const RINGER_SETTING: Uuid = Uuid::new_uuid16(0x2A41);
/// Alert Category ID Bit Mask
pub const ALERT_CATEGORY_ID_BIT_MASK: Uuid = Uuid::new_uuid16(0x2A42);
/// Alert Category ID
pub const ALERT_CATEGORY_ID: Uuid = Uuid::new_uuid16(0x2A43);
/// Alert Notification Control Point
pub const ALERT_NOTIFICATION_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A44);
/// Unread Alert Status
pub const UNREAD_ALERT_STATUS: Uuid = Uuid::new_uuid16(0x2A45);
/// New Alert
pub const NEW_ALERT: Uuid = Uuid::new_uuid16(0x2A46);
/// Supported New Alert Category
pub const SUPPORTED_NEW_ALERT_CATEGORY: Uuid = Uuid::new_uuid16(0x2A47);
/// Supported Unread Alert Category
pub const SUPPORTED_UNREAD_ALERT_CATEGORY: Uuid = Uuid::new_uuid16(0x2A48);
/// Blood Pressure Feature
pub const BLOOD_PRESSURE_FEATURE: Uuid = Uuid::new_uuid16(0x2A49);
/// HID Information
pub const HID_INFORMATION: Uuid = Uuid::new_uuid16(0x2A4A);
/// Report Map
pub const REPORT_MAP: Uuid = Uuid::new_uuid16(0x2A4B);
/// HID Control Point
pub const HID_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A4C);
/// Report
pub const REPORT: Uuid = Uuid::new_uuid16(0x2A4D);
/// Protocol Mode
pub const PROTOCOL_MODE: Uuid = Uuid::new_uuid16(0x2A4E);
/// Scan Interval Window
pub const SCAN_INTERVAL_WINDOW: Uuid = Uuid::new_uuid16(0x2A4F);
/// PnP ID
pub const PNP_ID: Uuid = Uuid::new_uuid16(0x2A50);
/// Glucose Feature
pub const GLUCOSE_FEATURE: Uuid = Uuid::new_uuid16(0x2A51);
/// Record Access Control Point
pub const RECORD_ACCESS_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A52);
/// RSC Measurement
pub const RSC_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A53);
/// RSC Feature
pub const RSC_FEATURE: Uuid = Uuid::new_uuid16(0x2A54);
/// SC Control Point
pub const SC_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A55);
/// Aggregate
pub const AGGREGATE: Uuid = Uuid::new_uuid16(0x2A5A);
/// CSC Measurement
pub const CSC_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A5B);
/// CSC Feature
pub const CSC_FEATURE: Uuid = Uuid::new_uuid16(0x2A5C);
/// Sensor Location
pub const SENSOR_LOCATION: Uuid = Uuid::new_uuid16(0x2A5D);
/// PLX Spot-Check Measurement
pub const PLX_SPOT_CHECK_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A5E);
/// PLX Continuous Measurement
pub const PLX_CONTINUOUS_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A5F);
/// PLX Features
pub const PLX_FEATURES: Uuid = Uuid::new_uuid16(0x2A60);
/// Cycling Power Measurement
pub const CYCLING_POWER_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A63);
/// Cycling Power Vector
pub const CYCLING_POWER_VECTOR: Uuid = Uuid::new_uuid16(0x2A64);
/// Cycling Power Feature
pub const CYCLING_POWER_FEATURE: Uuid = Uuid::new_uuid16(0x2A65);
/// Cycling Power Control Point
pub const CYCLING_POWER_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A66);
/// Location and Speed
pub const LOCATION_AND_SPEED: Uuid = Uuid::new_uuid16(0x2A67);
/// Navigation
pub const NAVIGATION: Uuid = Uuid::new_uuid16(0x2A68);
/// Position Quality
pub const POSITION_QUALITY: Uuid = Uuid::new_uuid16(0x2A69);
/// LN Feature
pub const LN_FEATURE: Uuid = Uuid::new_uuid16(0x2A6A);
/// LN Control Point
pub const LN_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A6B);
/// Elevation
pub const ELEVATION: Uuid = Uuid::new_uuid16(0x2A6C);
/// Pressure
pub const PRESSURE: Uuid = Uuid::new_uuid16(0x2A6D);
/// Temperature
pub const TEMPERATURE: Uuid = Uuid::new_uuid16(0x2A6E);
/// Humidity
pub const HUMIDITY: Uuid = Uuid::new_uuid16(0x2A6F);
/// True Wind Speed
pub const TRUE_WIND_SPEED: Uuid = Uuid::new_uuid16(0x2A70);
/// True Wind Direction
pub const TRUE_WIND_DIRECTION: Uuid = Uuid::new_uuid16(0x2A71);
/// Apparent Wind Speed
pub const APPARENT_WIND_SPEED: Uuid = Uuid::new_uuid16(0x2A72);
/// Apparent Wind Direction
pub const APPARENT_WIND_DIRECTION: Uuid = Uuid::new_uuid16(0x2A73);
/// Gust Factor
pub const GUST_FACTOR: Uuid = Uuid::new_uuid16(0x2A74);
/// Pollen Concentration
pub const POLLEN_CONCENTRATION: Uuid = Uuid::new_uuid16(0x2A75);
/// UV Index
pub const UV_INDEX: Uuid = Uuid::new_uuid16(0x2A76);
/// Irradiance
pub const IRRADIANCE: Uuid = Uuid::new_uuid16(0x2A77);
/// Rainfall
pub const RAINFALL: Uuid = Uuid::new_uuid16(0x2A78);
/// Wind Chill
pub const WIND_CHILL: Uuid = Uuid::new_uuid16(0x2A79);
/// Heat Index
pub const HEAT_INDEX: Uuid = Uuid::new_uuid16(0x2A7A);
/// Dew Point
pub const DEW_POINT: Uuid = Uuid::new_uuid16(0x2A7B);
/// Descriptor Value Changed
pub const DESCRIPTOR_VALUE_CHANGED: Uuid = Uuid::new_uuid16(0x2A7D);
/// Aerobic Heart Rate Lower Limit
pub const AEROBIC_HEART_RATE_LOWER_LIMIT: Uuid = Uuid::new_uuid16(0x2A7E);
/// Aerobic Threshold
pub const AEROBIC_THRESHOLD: Uuid = Uuid::new_uuid16(0x2A7F);
/// Age
pub const AGE: Uuid = Uuid::new_uuid16(0x2A80);
/// Anaerobic Heart Rate Lower Limit
pub const ANAEROBIC_HEART_RATE_LOWER_LIMIT: Uuid = Uuid::new_uuid16(0x2A81);
/// Anaerobic Heart Rate Upper Limit
pub const ANAEROBIC_HEART_RATE_UPPER_LIMIT: Uuid = Uuid::new_uuid16(0x2A82);
/// Anaerobic Threshold
pub const ANAEROBIC_THRESHOLD: Uuid = Uuid::new_uuid16(0x2A83);
/// Aerobic Heart Rate Upper Limit
pub const AEROBIC_HEART_RATE_UPPER_LIMIT: Uuid = Uuid::new_uuid16(0x2A84);
/// Date of Birth
pub const DATE_OF_BIRTH: Uuid = Uuid::new_uuid16(0x2A85);
/// Date of Threshold Assessment
pub const DATE_OF_THRESHOLD_ASSESSMENT: Uuid = Uuid::new_uuid16(0x2A86);
/// Email Address
pub const EMAIL_ADDRESS: Uuid = Uuid::new_uuid16(0x2A87);
/// Fat Burn Heart Rate Lower Limit
pub const FAT_BURN_HEART_RATE_LOWER_LIMIT: Uuid = Uuid::new_uuid16(0x2A88);
/// Fat Burn Heart Rate Upper Limit
pub const FAT_BURN_HEART_RATE_UPPER_LIMIT: Uuid = Uuid::new_uuid16(0x2A89);
/// First Name
pub const FIRST_NAME: Uuid = Uuid::new_uuid16(0x2A8A);
/// Five Zone Heart Rate Limits
pub const FIVE_ZONE_HEART_RATE_LIMITS: Uuid = Uuid::new_uuid16(0x2A8B);
/// Gender
pub const GENDER: Uuid = Uuid::new_uuid16(0x2A8C);
/// Heart Rate Max
pub const HEART_RATE_MAX: Uuid = Uuid::new_uuid16(0x2A8D);
/// Height
pub const HEIGHT: Uuid = Uuid::new_uuid16(0x2A8E);
/// Hip Circumference
pub const HIP_CIRCUMFERENCE: Uuid = Uuid::new_uuid16(0x2A8F);
/// Last Name
pub const LAST_NAME: Uuid = Uuid::new_uuid16(0x2A90);
/// Maximum Recommended Heart Rate
pub const MAXIMUM_RECOMMENDED_HEART_RATE: Uuid = Uuid::new_uuid16(0x2A91);
/// Resting Heart Rate
pub const RESTING_HEART_RATE: Uuid = Uuid::new_uuid16(0x2A92);
/// Sport Type for Aerobic and Anaerobic Thresholds
pub const SPORT_TYPE_FOR_AEROBIC_AND_ANAEROBIC_THRESHOLDS: Uuid = Uuid::new_uuid16(0x2A93);
/// Three Zone Heart Rate Limits
pub const THREE_ZONE_HEART_RATE_LIMITS: Uuid = Uuid::new_uuid16(0x2A94);
/// Two Zone Heart Rate Limits
pub const TWO_ZONE_HEART_RATE_LIMITS: Uuid = Uuid::new_uuid16(0x2A95);
/// VO2 Max
pub const VO2_MAX: Uuid = Uuid::new_uuid16(0x2A96);
/// Waist Circumference
pub const WAIST_CIRCUMFERENCE: Uuid = Uuid::new_uuid16(0x2A97);
/// Weight
pub const WEIGHT: Uuid = Uuid::new_uuid16(0x2A98);
/// Database Change Increment
pub const DATABASE_CHANGE_INCREMENT: Uuid = Uuid::new_uuid16(0x2A99);
/// User Index
pub const USER_INDEX: Uuid = Uuid::new_uuid16(0x2A9A);
/// Body Composition Feature
pub const BODY_COMPOSITION_FEATURE: Uuid = Uuid::new_uuid16(0x2A9B);
/// Body Composition Measurement
pub const BODY_COMPOSITION_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A9C);
/// Weight Measurement
pub const WEIGHT_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2A9D);
/// Weight Scale Feature
pub const WEIGHT_SCALE_FEATURE: Uuid = Uuid::new_uuid16(0x2A9E);
/// User Control Point
pub const USER_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2A9F);
/// Magnetic Flux Density - 2D
pub const MAGNETIC_FLUX_DENSITY_2D: Uuid = Uuid::new_uuid16(0x2AA0);
/// Magnetic Flux Density - 3D
pub const MAGNETIC_FLUX_DENSITY_3D: Uuid = Uuid::new_uuid16(0x2AA1);
/// Language
pub const LANGUAGE: Uuid = Uuid::new_uuid16(0x2AA2);
/// Barometric Pressure Trend
pub const BAROMETRIC_PRESSURE_TREND: Uuid = Uuid::new_uuid16(0x2AA3);
/// Bond Management Control Point
pub const BOND_MANAGEMENT_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2AA4);
/// Bond Management Feature
pub const BOND_MANAGEMENT_FEATURE: Uuid = Uuid::new_uuid16(0x2AA5);
/// Central Address Resolution
pub const CENTRAL_ADDRESS_RESOLUTION: Uuid = Uuid::new_uuid16(0x2AA6);
/// CGM Measurement
pub const CGM_MEASUREMENT: Uuid = Uuid::new_uuid16(0x2AA7);
/// CGM Feature
pub const CGM_FEATURE: Uuid = Uuid::new_uuid16(0x2AA8);
/// CGM Status
pub const CGM_STATUS: Uuid = Uuid::new_uuid16(0x2AA9);
/// CGM Session Start Time
pub const CGM_SESSION_START_TIME: Uuid = Uuid::new_uuid16(0x2AAA);
/// CGM Session Run Time
pub const CGM_SESSION_RUN_TIME: Uuid = Uuid::new_uuid16(0x2AAB);
/// CGM Specific Ops Control Point
pub const CGM_SPECIFIC_OPS_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2AAC);
/// Indoor Positioning Configuration
pub const INDOOR_POSITIONING_CONFIGURATION: Uuid = Uuid::new_uuid16(0x2AAD);
/// Latitude
pub const LATITUDE: Uuid = Uuid::new_uuid16(0x2AAE);
/// Longitude
pub const LONGITUDE: Uuid = Uuid::new_uuid16(0x2AAF);
/// Local North Coordinate
pub const LOCAL_NORTH_COORDINATE: Uuid = Uuid::new_uuid16(0x2AB0);
/// Local East Coordinate
pub const LOCAL_EAST_COORDINATE: Uuid = Uuid::new_uuid16(0x2AB1);
/// Floor Number
pub const FLOOR_NUMBER: Uuid = Uuid::new_uuid16(0x2AB2);
/// Altitude
pub const ALTITUDE: Uuid = Uuid::new_uuid16(0x2AB3);
/// Uncertainty
pub const UNCERTAINTY: Uuid = Uuid::new_uuid16(0x2AB4);
/// Location Name
pub const LOCATION_NAME: Uuid = Uuid::new_uuid16(0x2AB5);
/// URI
pub const URI: Uuid = Uuid::new_uuid16(0x2AB6);
/// HTTP Headers
pub const HTTP_HEADERS: Uuid = Uuid::new_uuid16(0x2AB7);
/// HTTP Status Code
pub const HTTP_STATUS_CODE: Uuid = Uuid::new_uuid16(0x2AB8);
/// HTTP Entity Body
pub const HTTP_ENTITY_BODY: Uuid = Uuid::new_uuid16(0x2AB9);
/// HTTP Control Point
pub const HTTP_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2ABA);
/// HTTPS Security
pub const HTTPS_SECURITY: Uuid = Uuid::new_uuid16(0x2ABB);
/// TDS Control Point
pub const TDS_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2ABC);
/// OTS Feature
pub const OTS_FEATURE: Uuid = Uuid::new_uuid16(0x2ABD);
/// object name
pub const OBJECT_NAME: Uuid = Uuid::new_uuid16(0x2ABE);
/// object type
pub const OBJECT_TYPE: Uuid = Uuid::new_uuid16(0x2ABF);
/// object size
pub const OBJECT_SIZE: Uuid = Uuid::new_uuid16(0x2AC0);
/// object first created
pub const OBJECT_FIRST_CREATED: Uuid = Uuid::new_uuid16(0x2AC1);
/// object last modified
pub const OBJECT_LAST_MODIFIED: Uuid = Uuid::new_uuid16(0x2AC2);
/// object ID
pub const OBJECT_ID: Uuid = Uuid::new_uuid16(0x2AC3);
/// object properties
pub const OBJECT_PROPERTIES: Uuid = Uuid::new_uuid16(0x2AC4);
/// object actioncontrol point
pub const OBJECT_ACTIONCONTROL_POINT: Uuid = Uuid::new_uuid16(0x2AC5);
/// object list control point
pub const OBJECT_LIST_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2AC6);
/// object list filter
pub const OBJECT_LIST_FILTER: Uuid = Uuid::new_uuid16(0x2AC7);
/// object changed
pub const OBJECT_CHANGED: Uuid = Uuid::new_uuid16(0x2AC8);
/// Resolvable Private Address Only
pub const RESOLVABLE_PRIVATE_ADDRESS_ONLY: Uuid = Uuid::new_uuid16(0x2AC9);
/// Unspecified
pub const UNSPECIFIED: Uuid = Uuid::new_uuid16(0x2ACA);
/// Directory Listing
pub const DIRECTORY_LISTING: Uuid = Uuid::new_uuid16(0x2ACB);
/// Fitness Machine Feature
pub const FITNESS_MACHINE_FEATURE: Uuid = Uuid::new_uuid16(0x2ACC);
/// Treadmill Data
pub const TREADMILL_DATA: Uuid = Uuid::new_uuid16(0x2ACD);
/// Cross Trainer Data
pub const CROSS_TRAINER_DATA: Uuid = Uuid::new_uuid16(0x2ACE);
/// Step Climber Data
pub const STEP_CLIMBER_DATA: Uuid = Uuid::new_uuid16(0x2ACF);
/// Stair Climber Data
pub const STAIR_CLIMBER_DATA: Uuid = Uuid::new_uuid16(0x2AD0);
/// Rower Data
pub const ROWER_DATA: Uuid = Uuid::new_uuid16(0x2AD1);
/// Indoor Bike Data
pub const INDOOR_BIKE_DATA: Uuid = Uuid::new_uuid16(0x2AD2);
/// Training Status
pub const TRAINING_STATUS: Uuid = Uuid::new_uuid16(0x2AD3);
/// Supported Speed Range
pub const SUPPORTED_SPEED_RANGE: Uuid = Uuid::new_uuid16(0x2AD4);
/// Supported Inclination Range
pub const SUPPORTED_INCLINATION_RANGE: Uuid = Uuid::new_uuid16(0x2AD5);
/// Supported Resistance Level Range
pub const SUPPORTED_RESISTANCE_LEVEL_RANGE: Uuid = Uuid::new_uuid16(0x2AD6);
/// Supported Heart Rate Range
pub const SUPPORTED_HEART_RATE_RANGE: Uuid = Uuid::new_uuid16(0x2AD7);
/// Supported Power Range
pub const SUPPORTED_POWER_RANGE: Uuid = Uuid::new_uuid16(0x2AD8);
/// Fitness Machine Control Point
pub const FITNESS_MACHINE_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2AD9);
/// Fitness Machine Status
pub const FITNESS_MACHINE_STATUS: Uuid = Uuid::new_uuid16(0x2ADA);
/// Mesh Provisioning Data In
pub const MESH_PROVISIONING_DATA_IN: Uuid = Uuid::new_uuid16(0x2ADB);
/// Mesh Provisioning Data Out
pub const MESH_PROVISIONING_DATA_OUT: Uuid = Uuid::new_uuid16(0x2ADC);
/// Mesh Proxy Data In
pub const MESH_PROXY_DATA_IN: Uuid = Uuid::new_uuid16(0x2ADD);
/// Mesh Proxy Data Out
pub const MESH_PROXY_DATA_OUT: Uuid = Uuid::new_uuid16(0x2ADE);
/// Average Current
pub const AVERAGE_CURRENT: Uuid = Uuid::new_uuid16(0x2AE0);
/// Average Voltage
pub const AVERAGE_VOLTAGE: Uuid = Uuid::new_uuid16(0x2AE1);
/// Boolean
pub const BOOLEAN: Uuid = Uuid::new_uuid16(0x2AE2);
/// Chromatic Distance From Planckian
pub const CHROMATIC_DISTANCE_FROM_PLANCKIAN: Uuid = Uuid::new_uuid16(0x2AE3);
/// Chromaticity Coordinates
pub const CHROMATICITY_COORDINATES: Uuid = Uuid::new_uuid16(0x2AE4);
/// Chromaticity in CCT And Duv Values
pub const CHROMATICITY_IN_CCT_AND_DUV_VALUES: Uuid = Uuid::new_uuid16(0x2AE5);
/// Chromaticity Tolerance
pub const CHROMATICITY_TOLERANCE: Uuid = Uuid::new_uuid16(0x2AE6);
/// CIE 13.3-1995 Color Rendering Index
pub const CIE_13_3_1995_COLOR_RENDERING_INDEX: Uuid = Uuid::new_uuid16(0x2AE7);
/// Coefficient
pub const COEFFICIENT: Uuid = Uuid::new_uuid16(0x2AE8);
/// Correlated Color Temperature
pub const CORRELATED_COLOR_TEMPERATURE: Uuid = Uuid::new_uuid16(0x2AE9);
/// Count 16
pub const COUNT_16: Uuid = Uuid::new_uuid16(0x2AEA);
/// Count 24
pub const COUNT_24: Uuid = Uuid::new_uuid16(0x2AEB);
/// Country Code
pub const COUNTRY_CODE: Uuid = Uuid::new_uuid16(0x2AEC);
/// Date UTC
pub const DATE_UTC: Uuid = Uuid::new_uuid16(0x2AED);
/// Electric Current
pub const ELECTRIC_CURRENT: Uuid = Uuid::new_uuid16(0x2AEE);
/// Electric Current Range
pub const ELECTRIC_CURRENT_RANGE: Uuid = Uuid::new_uuid16(0x2AEF);
/// Electric Current Specification
pub const ELECTRIC_CURRENT_SPECIFICATION: Uuid = Uuid::new_uuid16(0x2AF0);
/// Electric Current Statistics
pub const ELECTRIC_CURRENT_STATISTICS: Uuid = Uuid::new_uuid16(0x2AF1);
/// Energy
pub const ENERGY: Uuid = Uuid::new_uuid16(0x2AF2);
/// Energy In A Period Of Day
pub const ENERGY_IN_A_PERIOD_OF_DAY: Uuid = Uuid::new_uuid16(0x2AF3);
/// Event Statistics
pub const EVENT_STATISTICS: Uuid = Uuid::new_uuid16(0x2AF4);
/// Fixed String 16
pub const FIXED_STRING_16: Uuid = Uuid::new_uuid16(0x2AF5);
/// Fixed String 24
pub const FIXED_STRING_24: Uuid = Uuid::new_uuid16(0x2AF6);
/// Fixed String 36
pub const FIXED_STRING_36: Uuid = Uuid::new_uuid16(0x2AF7);
/// Fixed String 8
pub const FIXED_STRING_8: Uuid = Uuid::new_uuid16(0x2AF8);
/// Generic Level
pub const GENERIC_LEVEL: Uuid = Uuid::new_uuid16(0x2AF9);
/// Global Trade Item Number
pub const GLOBAL_TRADE_ITEM_NUMBER: Uuid = Uuid::new_uuid16(0x2AFA);
/// Illuminance
pub const ILLUMINANCE: Uuid = Uuid::new_uuid16(0x2AFB);
/// Luminous Efficacy
pub const LUMINOUS_EFFICACY: Uuid = Uuid::new_uuid16(0x2AFC);
/// Luminous Energy
pub const LUMINOUS_ENERGY: Uuid = Uuid::new_uuid16(0x2AFD);
/// Luminous Exposure
pub const LUMINOUS_EXPOSURE: Uuid = Uuid::new_uuid16(0x2AFE);
/// Luminous Flux
pub const LUMINOUS_FLUX: Uuid = Uuid::new_uuid16(0x2AFF);
/// Luminous Flux Range
pub const LUMINOUS_FLUX_RANGE: Uuid = Uuid::new_uuid16(0x2B00);
/// Luminous Intensity
pub const LUMINOUS_INTENSITY: Uuid = Uuid::new_uuid16(0x2B01);
/// Mass Flow
pub const MASS_FLOW: Uuid = Uuid::new_uuid16(0x2B02);
/// Perceived Lightness
pub const PERCEIVED_LIGHTNESS: Uuid = Uuid::new_uuid16(0x2B03);
/// Percentage 8
pub const PERCENTAGE_8: Uuid = Uuid::new_uuid16(0x2B04);
/// Power
pub const POWER: Uuid = Uuid::new_uuid16(0x2B05);
/// Power Specification
pub const POWER_SPECIFICATION: Uuid = Uuid::new_uuid16(0x2B06);
/// Relative Runtime In A Current Range
pub const RELATIVE_RUNTIME_IN_A_CURRENT_RANGE: Uuid = Uuid::new_uuid16(0x2B07);
/// Relative Runtime In A Generic Level Range
pub const RELATIVE_RUNTIME_IN_A_GENERIC_LEVEL_RANGE: Uuid = Uuid::new_uuid16(0x2B08);
/// Relative Value In A Voltage Range
pub const RELATIVE_VALUE_IN_A_VOLTAGE_RANGE: Uuid = Uuid::new_uuid16(0x2B09);
/// Relative Value In An Illuminance Range
pub const RELATIVE_VALUE_IN_AN_ILLUMINANCE_RANGE: Uuid = Uuid::new_uuid16(0x2B0A);
/// Relative Value In A Period Of Day
pub const RELATIVE_VALUE_IN_A_PERIOD_OF_DAY: Uuid = Uuid::new_uuid16(0x2B0B);
/// Relative Value In A Temperature Range
pub const RELATIVE_VALUE_IN_A_TEMPERATURE_RANGE: Uuid = Uuid::new_uuid16(0x2B0C);
/// Temperature 8
pub const TEMPERATURE_8: Uuid = Uuid::new_uuid16(0x2B0D);
/// Temperature 8 In A Period Of Day
pub const TEMPERATURE_8_IN_A_PERIOD_OF_DAY: Uuid = Uuid::new_uuid16(0x2B0E);
/// Temperature 8 Statistics
pub const TEMPERATURE_8_STATISTICS: Uuid = Uuid::new_uuid16(0x2B0F);
/// Temperature Range
pub const TEMPERATURE_RANGE: Uuid = Uuid::new_uuid16(0x2B10);
/// Temperature Statistics
pub const TEMPERATURE_STATISTICS: Uuid = Uuid::new_uuid16(0x2B11);
/// Time Decihour 8
pub const TIME_DECIHOUR_8: Uuid = Uuid::new_uuid16(0x2B12);
/// Time Exponential 8
pub const TIME_EXPONENTIAL_8: Uuid = Uuid::new_uuid16(0x2B13);
/// Time Hour 24
pub const TIME_HOUR_24: Uuid = Uuid::new_uuid16(0x2B14);
/// Time Millisecond 24
pub const TIME_MILLISECOND_24: Uuid = Uuid::new_uuid16(0x2B15);
/// Time Second 16
pub const TIME_SECOND_16: Uuid = Uuid::new_uuid16(0x2B16);
/// Time Second 8
pub const TIME_SECOND_8: Uuid = Uuid::new_uuid16(0x2B17);
/// Voltage
pub const VOLTAGE: Uuid = Uuid::new_uuid16(0x2B18);
/// Voltage Specification
pub const VOLTAGE_SPECIFICATION: Uuid = Uuid::new_uuid16(0x2B19);
/// Voltage Statistics
pub const VOLTAGE_STATISTICS: Uuid = Uuid::new_uuid16(0x2B1A);
/// Volume Flow
pub const VOLUME_FLOW: Uuid = Uuid::new_uuid16(0x2B1B);
/// Chromaticity Coordinate
pub const CHROMATICITY_COORDINATE: Uuid = Uuid::new_uuid16(0x2B1C);
/// RC Feature
pub const RC_FEATURE: Uuid = Uuid::new_uuid16(0x2B1D);
/// RC Settings
pub const RC_SETTINGS: Uuid = Uuid::new_uuid16(0x2B1E);
/// Reconnection Configuration Control Point
pub const RECONNECTION_CONFIGURATION_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2B1F);
/// IDD Status Changed
pub const IDD_STATUS_CHANGED: Uuid = Uuid::new_uuid16(0x2B20);
/// IDD Status
pub const IDD_STATUS: Uuid = Uuid::new_uuid16(0x2B21);
/// IDD Annunciation Status
pub const IDD_ANNUNCIATION_STATUS: Uuid = Uuid::new_uuid16(0x2B22);
/// IDD Features
pub const IDD_FEATURES: Uuid = Uuid::new_uuid16(0x2B23);
/// IDD Status Reader Control Point
pub const IDD_STATUS_READER_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2B24);
/// IDD Command Control Point
pub const IDD_COMMAND_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2B25);
/// IDD Command Data
pub const IDD_COMMAND_DATA: Uuid = Uuid::new_uuid16(0x2B26);
/// IDD Record Access Control Point
pub const IDD_RECORD_ACCESS_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2B27);
/// IDD History Data
pub const IDD_HISTORY_DATA: Uuid = Uuid::new_uuid16(0x2B28);
/// Client Supported Features
pub const CLIENT_SUPPORTED_FEATURES: Uuid = Uuid::new_uuid16(0x2B29);
/// Database Hash
pub const DATABASE_HASH: Uuid = Uuid::new_uuid16(0x2B2A);
/// BSS Control Point
pub const BSS_CONTROL_POINT: Uuid = Uuid::new_uuid16(0x2B2B);
/// BSS Response
pub const BSS_RESPONSE: Uuid = Uuid::new_uuid16(0x2B2C);
/// Emergency ID
pub const EMERGENCY_ID: Uuid = Uuid::new_uuid16(0x2B2D);
/// Emergency Text
pub const EMERGENCY_TEXT: Uuid = Uuid::new_uuid16(0x2B2E);
/// Server Supported Feature
pub const SERVER_SUPPORTED_FEATURE: Uuid = Uuid::new_uuid16(0x2B3A);
