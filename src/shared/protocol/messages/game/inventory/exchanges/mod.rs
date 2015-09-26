use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::data::items::ObjectItemToSellInBid; use protocol::types::game::data::items::SellerBuyerDescriptor; use protocol::variants::ObjectEffectVariant; use protocol::types::game::data::items::ObjectItemToSellInHumanVendorShop; use protocol::types::game::data::items::BidExchangerObjectInfo; use protocol::variants::UpdateMountBoostVariant; use protocol::messages::game::dialog::LeaveDialogMessage; use protocol::types::game::context::roleplay::job::DecraftedItemStackInfo; use protocol::types::game::data::items::ObjectItemNotInContainer; use protocol::types::game::data::items::ObjectItem; use protocol::types::game::data::items::ObjectItemToSell; use protocol::types::game::mount::MountClientData; use protocol::types::game::data::items::ObjectItemGenericQuantity; use protocol::types::game::data::items::ObjectItemToSellInNpcShop;
impl_type!(DecraftResultMessage, 6569, results| Vec<DecraftedItemStackInfo>);
impl_type!(ExchangeAcceptMessage, 5508);
impl_type!(ExchangeBidHouseBuyMessage, 5804, uid| VarInt, qty| VarInt, price| VarInt);
impl_type!(ExchangeBidHouseBuyResultMessage, 6272, uid| VarInt, bought| bool);
impl_type!(ExchangeBidHouseGenericItemAddedMessage, 5947, obj_generic_id| VarShort);
impl_type!(ExchangeBidHouseGenericItemRemovedMessage, 5948, obj_generic_id| VarShort);
impl_type!(ExchangeBidHouseInListAddedMessage, 5949, item_uid| i32, obj_generic_id| i32, effects| Vec<ObjectEffectVariant>, prices| Vec<VarInt>);
impl_type!(ExchangeBidHouseInListRemovedMessage, 5950, item_uid| i32);
impl_type!(ExchangeBidHouseInListUpdatedMessage, 6337, base| ExchangeBidHouseInListAddedMessage);
impl_type!(ExchangeBidHouseItemAddOkMessage, 5945, item_info| ObjectItemToSellInBid);
impl_type!(ExchangeBidHouseItemRemoveOkMessage, 5946, seller_id| i32);
impl_type!(ExchangeBidHouseListMessage, 5807, id| VarShort);
impl_type!(ExchangeBidHousePriceMessage, 5805, gen_id| VarShort);
impl_type!(ExchangeBidHouseSearchMessage, 5806, type_| VarInt, gen_id| VarShort);
impl_type!(ExchangeBidHouseTypeMessage, 5803, type_| VarInt);
impl_type!(ExchangeBidPriceForSellerMessage, 6464, base| ExchangeBidPriceMessage, all_identical| bool, minimal_prices| Vec<VarInt>);
impl_type!(ExchangeBidPriceMessage, 5755, generic_id| VarShort, average_price| VarInt);
impl_type!(ExchangeBidSearchOkMessage, 5802);
impl_type!(ExchangeBuyMessage, 5774, object_to_buy_id| VarInt, quantity| VarInt);
impl_type!(ExchangeBuyOkMessage, 5759);
impl_type!(ExchangeCraftCountModifiedMessage, 6595, count| VarInt);
impl_type!(ExchangeCraftCountRequestMessage, 6597, count| VarInt);
impl_type!(ExchangeCrafterJobLevelupMessage, 6598, crafter_job_level| i8);
impl_type!(ExchangeCraftInformationObjectMessage, 5794, base| ExchangeCraftResultWithObjectIdMessage, player_id| VarInt);
impl_type!(ExchangeCraftPaymentModificationRequestMessage, 6579, quantity| VarInt);
impl_type!(ExchangeCraftPaymentModifiedMessage, 6578, gold_sum| VarInt);
impl_type!(ExchangeCraftResultMagicWithObjectDescMessage, 6188, base| ExchangeCraftResultWithObjectDescMessage, magic_pool_status| i8);
impl_type!(ExchangeCraftResultMessage, 5790, craft_result| i8);
impl_type!(ExchangeCraftResultWithObjectDescMessage, 5999, base| ExchangeCraftResultMessage, object_info| ObjectItemNotInContainer);
impl_type!(ExchangeCraftResultWithObjectIdMessage, 6000, base| ExchangeCraftResultMessage, object_generic_id| VarShort);
impl_type!(ExchangeErrorMessage, 5513, error_type| i8);
impl_type!(ExchangeGuildTaxCollectorGetMessage, 5762, collector_name| String, world_x| i16, world_y| i16, map_id| i32, sub_area_id| VarShort, user_name| String, experience| f64, objects_infos| Vec<ObjectItemGenericQuantity>);
impl_type!(ExchangeHandleMountsStableMessage, 6562, action_type| i8, rides_id| Vec<VarInt>);
impl_type!(ExchangeIsReadyMessage, 5509, id| VarInt, ready| bool);
impl_type!(ExchangeItemAutoCraftStopedMessage, 5810, reason| i8);
impl_type!(ExchangeLeaveMessage, 5628, base| LeaveDialogMessage, success| bool);
impl_type!(ExchangeMountFreeFromPaddockMessage, 6055, name| String, world_x| i16, world_y| i16, liberator| String);
impl_type!(ExchangeMountsPaddockAddMessage, 6561, mount_description| Vec<MountClientData>);
impl_type!(ExchangeMountsPaddockRemoveMessage, 6559, mounts_id| Vec<VarInt>);
impl_type!(ExchangeMountsStableAddMessage, 6555, mount_description| Vec<MountClientData>);
impl_type!(ExchangeMountsStableBornAddMessage, 6557, base| ExchangeMountsStableAddMessage);
impl_type!(ExchangeMountsStableRemoveMessage, 6556, mounts_id| Vec<VarInt>);
impl_type!(ExchangeMountStableErrorMessage, 5981);
impl_type!(ExchangeMountsTakenFromPaddockMessage, 6554, name| String, world_x| i16, world_y| i16, ownername| String);
impl_type!(ExchangeMountSterilizeFromPaddockMessage, 6056, name| String, world_x| i16, world_y| i16, sterilizator| String);
impl_type!(ExchangeObjectAddedMessage, 5516, base| ExchangeObjectMessage, object| ObjectItem);
impl_type!(ExchangeObjectMessage, 5515, remote| bool);
impl_type!(ExchangeObjectModifyPricedMessage, 6238, base| ExchangeObjectMovePricedMessage);
impl_type!(ExchangeObjectMoveKamaMessage, 5520, quantity| VarInt);
impl_type!(ExchangeObjectMoveMessage, 5518, object_uid| VarInt, quantity| VarInt);
impl_type!(ExchangeObjectMovePricedMessage, 5514, base| ExchangeObjectMoveMessage, price| VarInt);
impl_type!(ExchangeObjectsAddedMessage, 6535, base| ExchangeObjectMessage, object| Vec<ObjectItem>);
impl_type!(ExchangeObjectTransfertAllFromInvMessage, 6184);
impl_type!(ExchangeObjectTransfertAllToInvMessage, 6032);
impl_type!(ExchangeObjectTransfertExistingFromInvMessage, 6325);
impl_type!(ExchangeObjectTransfertExistingToInvMessage, 6326);
impl_type!(ExchangeObjectTransfertListFromInvMessage, 6183, ids| Vec<VarInt>);
impl_type!(ExchangeObjectTransfertListToInvMessage, 6039, ids| Vec<VarInt>);
impl_type!(ExchangeObjectTransfertListWithQuantityToInvMessage, 6470, ids| Vec<VarInt>, qtys| Vec<VarInt>);
impl_type!(ExchangeObjectUseInWorkshopMessage, 6004, object_uid| VarInt, quantity| VarInt);
impl_type!(ExchangeOkMultiCraftMessage, 5768, initiator_id| VarInt, other_id| VarInt, role| i8);
impl_type!(ExchangeOnHumanVendorRequestMessage, 5772, human_vendor_id| VarInt, human_vendor_cell| VarShort);
impl_type!(ExchangePlayerMultiCraftRequestMessage, 5784, base| ExchangeRequestMessage, target| VarInt, skill_id| VarInt);
impl_type!(ExchangePlayerRequestMessage, 5773, base| ExchangeRequestMessage, target| VarInt);
impl_type!(ExchangeReadyMessage, 5511, ready| bool, step| VarShort);
impl_type!(ExchangeReplayStopMessage, 6001);
impl_type!(ExchangeReplyTaxVendorMessage, 5787, object_value| VarInt, total_tax_value| VarInt);
impl_type!(ExchangeRequestedMessage, 5522, exchange_type| i8);
impl_type!(ExchangeRequestedTradeMessage, 5523, base| ExchangeRequestedMessage, source| VarInt, target| VarInt);
impl_type!(ExchangeRequestMessage, 5505, exchange_type| i8);
impl_type!(ExchangeRequestOnMountStockMessage, 5986);
impl_type!(ExchangeRequestOnShopStockMessage, 5753);
impl_type!(ExchangeRequestOnTaxCollectorMessage, 5779, tax_collector_id| i32);
impl_type!(ExchangeSellMessage, 5778, object_to_sell_id| VarInt, quantity| VarInt);
impl_type!(ExchangeSellOkMessage, 5792);
impl_type!(ExchangeSetCraftRecipeMessage, 6389, object_gid| VarShort);
impl_type!(ExchangeShopStockMovementRemovedMessage, 5907, object_id| VarInt);
impl_type!(ExchangeShopStockMovementUpdatedMessage, 5909, object_info| ObjectItemToSell);
impl_type!(ExchangeShopStockMultiMovementRemovedMessage, 6037, object_id_list| Vec<VarInt>);
impl_type!(ExchangeShopStockMultiMovementUpdatedMessage, 6038, object_info_list| Vec<ObjectItemToSell>);
impl_type!(ExchangeShopStockStartedMessage, 5910, objects_infos| Vec<ObjectItemToSell>);
impl_type!(ExchangeShowVendorTaxMessage, 5783);
impl_type!(ExchangeStartAsVendorMessage, 5775);
impl_type!(ExchangeStartedBidBuyerMessage, 5904, buyer_descriptor| SellerBuyerDescriptor);
impl_type!(ExchangeStartedBidSellerMessage, 5905, seller_descriptor| SellerBuyerDescriptor, objects_infos| Vec<ObjectItemToSellInBid>);
impl_type!(ExchangeStartedMessage, 5512, exchange_type| i8);
impl_type!(ExchangeStartedMountStockMessage, 5984, objects_infos| Vec<ObjectItem>);
impl_type!(ExchangeStartedWithPodsMessage, 6129, base| ExchangeStartedMessage, first_character_id| i32, first_character_current_weight| VarInt, first_character_max_weight| VarInt, second_character_id| i32, second_character_current_weight| VarInt, second_character_max_weight| VarInt);
impl_type!(ExchangeStartedWithStorageMessage, 6236, base| ExchangeStartedMessage, storage_max_slot| VarInt);
impl_type!(ExchangeStartOkCraftMessage, 5813);
impl_type!(ExchangeStartOkCraftWithInformationMessage, 5941, base| ExchangeStartOkCraftMessage, skill_id| VarInt);
impl_type!(ExchangeStartOkHumanVendorMessage, 5767, seller_id| VarInt, objects_infos| Vec<ObjectItemToSellInHumanVendorShop>);
impl_type!(ExchangeStartOkJobIndexMessage, 5819, jobs| Vec<VarInt>);
impl_type!(ExchangeStartOkMountMessage, 5979, base| ExchangeStartOkMountWithOutPaddockMessage, paddocked_mounts_description| Vec<MountClientData>);
impl_type!(ExchangeStartOkMountWithOutPaddockMessage, 5991, stabled_mounts_description| Vec<MountClientData>);
impl_type!(ExchangeStartOkMulticraftCrafterMessage, 5818, skill_id| VarInt);
impl_type!(ExchangeStartOkMulticraftCustomerMessage, 5817, skill_id| VarInt, crafter_job_level| i8);
impl_type!(ExchangeStartOkNpcShopMessage, 5761, npc_seller_id| i32, token_id| VarShort, objects_infos| Vec<ObjectItemToSellInNpcShop>);
impl_type!(ExchangeStartOkNpcTradeMessage, 5785, npc_id| i32);
impl_type!(ExchangeStartOkRecycleTradeMessage, 6600, percent_to_prism| i16, percent_to_player| i16);
impl_type!(ExchangeStartOkRunesTradeMessage, 6567);
impl_type!(ExchangeStoppedMessage, 6589, id| VarInt);
impl_type!(ExchangeTypesExchangerDescriptionForUserMessage, 5765, type_description| Vec<VarInt>);
impl_type!(ExchangeTypesItemsExchangerDescriptionForUserMessage, 5752, item_type_descriptions| Vec<BidExchangerObjectInfo>);
impl_type!(ExchangeWaitingResultMessage, 5786, bwait| bool);
impl_type!(ExchangeWeightMessage, 5793, current_weight| VarInt, max_weight| VarInt);
impl_type!(ItemNoMoreAvailableMessage, 5769);
impl_type!(JobBookSubscribeRequestMessage, 6592, job_id| i8);
impl_type!(RecycleResultMessage, 6601, nuggets_for_prism| VarInt, nuggets_for_player| VarInt);
impl_type!(UpdateMountBoostMessage, 6179, ride_id| VarInt, boost_to_update_list| Vec<UpdateMountBoostVariant>);
