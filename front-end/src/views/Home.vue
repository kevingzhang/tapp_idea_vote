<template>
<div class="tea-page">
  <h4>Welcome to Idea voter tapp.</h4>
  <p>Details information about tapp...</p>

  <el-button type="primary" @click="send_request()">Ping tapp</el-button>
</div>
</template>
<script>
import {_} from 'tearust_utils';
import layer2 from '../layer2';
export default {
  data(){
    return {};
  },
  methods: {
    async send_request(){
      const _axios = layer2.base.getAxios();
      try{
        const rs = await _axios.post('/ping', {
          sample: true,
          address: '0x000000000000000000000000000000000000000f'
        });
        alert(JSON.stringify(rs));
      }catch(e){
        alert(e.toString());
      }
      
    }
  }
};
</script>