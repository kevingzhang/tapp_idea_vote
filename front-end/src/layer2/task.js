import { _, uuid, moment } from 'tearust_utils';
import utils from '../tea/utils';
import store from '../store';
import { stringToHex, hexToU8a, stringToU8a, numberToHex, u8aToString } from '@polkadot/util';

import base from './base';
import txn from './txn';
import eth from '../eth';
import common from './common';
import user from './user';


const F = {
  async initDB(self, succ_cb){
    self.$root.loading(true);
    try {
      await txn.txn_request('init_db', {
        tokenId: base.getTappId(),
        address: self.layer1_account.address,
      });
      self.$root.alert_success();
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
      
    }
    self.$root.loading(false);
  },
  async initToken(self, succ_cb){
    self.$root.loading(true);
    try {
      await txn.txn_request('init_token', {
        tokenId: base.getTappId(),
        address: self.layer1_account.address,
      });
      self.$root.alert_success();
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);
  },

  async createNewIdea(self, param={}){
    const session_key = user.checkLogin(self);

    self.$root.loading(true);
    const tappId = base.getTappId();
    const amount = utils.layer1.amountToBalance(param.unit);
    const opts = {
      address: self.layer1_account.address,
      tappIdB64: tappId,
      authB64: session_key,
      id: uuid(),
      title: param.title,
      description: param.description,
      unit: utils.toBN(amount).toString(),
    };

    try {
      await txn.txn_request('create_idea', opts);
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);

  },
  
  async voteIdea(self, param, succ_cb){
    const session_key = user.checkLogin(self);
    const amt = utils.layer1.balanceToAmount(param.unit);
    try{
      await self.$confirm('Are you sure to vote this idea with '+amt+' TEA', {
        title: 'Vote idea',
        dangerouslyUseHTMLString: true,
      });
    }catch(e){
      return;
    }

    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
      id: param.id,
    };

    try {
      await txn.txn_request('vote_idea', opts);
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);
  },
  
  
  async queryIdeaList(self){
    const rs = await txn.query_request('query_idea_list', {
      address: self.layer1_account.address,
    });
    const list = _.map(rs.list, (item)=>{
      item.title = decodeURIComponent(utils.forge.util.decode64(item.title));
      item.description = decodeURIComponent(utils.forge.util.decode64(item.description));
      item.create_at = moment.utc(item.create_at*1000).format('YYYY-MM-DD hh:mm');
      item.total = utils.layer1.balanceToAmount(item.unit) * item.vote_num;
      return item;
    });

    return _.reverse(_.sortBy(list, 'total'));
  }


};

export default F;