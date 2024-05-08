/*
 Navicat Premium Data Transfer

 Source Server         : localPostgre
 Source Server Type    : PostgreSQL
 Source Server Version : 130012
 Source Host           : 127.0.0.1:5432
 Source Catalog        : gateway
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 130012
 File Encoding         : 65001

 Date: 06/05/2024 17:48:24
*/


-- ----------------------------
-- Table structure for clock_message
-- ----------------------------
DROP TABLE IF EXISTS "public"."clock_message";
CREATE TABLE "public"."clock_message" (
  "message_id" text COLLATE "pg_catalog"."default" NOT NULL,
  "node_id" text COLLATE "pg_catalog"."default" NOT NULL,
  "clock" json NOT NULL,
  "event_count" int4 NOT NULL,
  "is_zk" bool NOT NULL,
  "from_addr" text COLLATE "pg_catalog"."default",
  "to_addr" text COLLATE "pg_catalog"."default",
  "signature" text COLLATE "pg_catalog"."default" NOT NULL,
  "raw_message" varchar(255) COLLATE "pg_catalog"."default",
  "create_at" timestamp(6) NOT NULL
)
;
ALTER TABLE "public"."clock_message" OWNER TO "postgres";

-- ----------------------------
-- Records of clock_message
-- ----------------------------
BEGIN;
INSERT INTO "public"."clock_message" VALUES ('msg0003', '0x679320A64036b710371374aEdfa59Cff5c16f1CA', '{"0x679320A64036b710371374aEdfa59Cff5c16f1CA": 3}', 3, 't', '0xF5cF915C7B16a72f265F2531f87D2f4A3b7F3E29', '0xCd29894dA4825c87277B32Fd6855B72e30489AD9', 'sigasdasdadasdasdasd', 'hello', '2024-04-27 04:33:24.590085');
INSERT INTO "public"."clock_message" VALUES ('msg0002', '0x679320A64036b710371374aEdfa59Cff5c16f1CA', '{"0x679320A64036b710371374aEdfa59Cff5c16f1CA": 2}', 2, 't', '0xF5cF915C7B16a72f265F2531f87D2f4A3b7F3E29', '0xCd29894dA4825c87277B32Fd6855B72e30489AD9', 'sigasdasdadasdasdasd', 'hello', '2024-04-27 04:33:24.588606');
INSERT INTO "public"."clock_message" VALUES ('msg0001', '0x679320A64036b710371374aEdfa59Cff5c16f1CA', '{"0x679320A64036b710371374aEdfa59Cff5c16f1CA": 1}', 1, 't', '0xF5cF915C7B16a72f265F2531f87D2f4A3b7F3E29', '0xCd29894dA4825c87277B32Fd6855B72e30489AD9', 'sigasdasdadasdasdasd', 'hello', '2024-04-27 04:33:24.584489');
COMMIT;

-- ----------------------------
-- Primary Key structure for table clock_message
-- ----------------------------
ALTER TABLE "public"."clock_message" ADD CONSTRAINT "clock_message_pkey" PRIMARY KEY ("message_id");
