<template>
<div class="tea-page">
  <h4>Idea list</h4>

  <el-button size="small" class="tea-refresh-btn" type="primary" plain icon="el-icon-refresh" circle @click="refreshList()"></el-button>
  <TeaTable
    :data="list || []"
    name="idea_list_table"
  >
    <!-- <el-table-column
      prop="id"
      label="Idea id"
    /> -->

    

    <el-table-column
      label="Title"
    >
      <template slot-scope="scope">
        <span :inner-html.prop="scope.row.title"></span>
      </template>
    </el-table-column>

    <el-table-column
      prop="owner"
      label="Creator"
    />

    <el-table-column
      label="Deposit"
    >
      <template slot-scope="scope">
        <span :inner-html.prop="scope.row.total | teaIcon"></span>
      </template>
    </el-table-column>

    <el-table-column
      prop="vote_num"
      label="Vote count"
    />
    

    <el-table-column
      prop="create_at"
      label="Created at"
    />

    <el-table-column
      label="Actions"
      width="100"
      fixed="right"
    >
      <template slot-scope="scope">
        
        <TeaIconButton icon="NA" title="View" @click="viewIdea(scope.row)" />
        <TeaIconButton icon="NA" title="Vote" @click="voteIdea(scope.row)" />
        
      </template>
    </el-table-column>


  </TeaTable>

  <div class="tea-legend" style="
    margin-top: 40px;
    position: relative;
  ">

    <el-button style="width:400px;position:absolute;top:0; right:0;" type="primary" @click="createNewIdeaModal()">Create new idea</el-button>
  </div>
  
  <el-dialog
    title="Create idea"
    :visible="modal.visible"
    width="80%"
    :close-on-click-modal="false"
    custom-class="tea-modal"
    :destroy-on-close="true"
    @opened="openHandler()"
    @close="close()"
  >
    <div>
      <div><el-input type="text" v-model="modal.title" placeholder="Idea title"></el-input></div>
      <div style="margin-top: 30px;" ><el-input-number v-model="modal.unit" :min="1" :step="1" :max="100" placeholder="Vote price"></el-input-number><label style="margin-left: 15px;display: inline-block;">( Vote unit price )</label></div>
      <vue-ckeditor 
        style="margin-top: 30px;"
        v-model="modal.description" 
        :config="modal.config" 
        @blur="onBlur($event)" 
        @focus="onFocus($event)"
        @contentDom="onContentDom($event)"
        @dialogDefinition="onDialogDefinition($event)" />
    </div>




    <span slot="footer" class="dialog-footer">
      <el-button size="small" @click="close()">Cancel</el-button>
      <el-button size="small" type="primary" @click="createNewIdea()">
        Confirm
      </el-button>
    </span>

  </el-dialog>

  <el-dialog
    title="Idea details"
    :visible="vw.visible"
    width="80%"
    :close-on-click-modal="true"
    custom-class="tea-modal"
    :destroy-on-close="true"
    @close="vw.visible=false"
  >
    <div v-if="vw.visible">
      <h4 v-html="vw.title" style="text-align:center;margin-top:0;font-size: 19px;"></h4>
      <hr style="height: 1px; background: #ddd; border: none;" />
      <div v-html="vw.description"></div>
    </div>

  </el-dialog>

</div>
</template>
<script>
import Base from '../workflow/Base';
import {_} from 'tearust_utils';
import utils from '../tea/utils';
import { mapGetters, mapState } from 'vuex';
import TeaTable from '../components/TeaTable';
import TeaTableColumn from '../components/TeaTableColumn';
import TeaIconButton from '../components/TeaIconButton';
import layer2 from '../layer2';
import VueCkeditor from 'vue-ckeditor2';


export default {
  components: {
    TeaTable,
    TeaIconButton,
    TeaTableColumn,
    VueCkeditor,
  },
  data(){
    return {
      list: null,
      modal: {
        visible: false,
        title: '',
        description: '',
        unit: 1,
        config: {
          toolbar: [
            { name: 'clipboard', items: [ 'Cut', 'Copy', 'Paste', 'PasteText', 'PasteFromWord', '-', 'Undo', 'Redo' ] },
            { name: 'basicstyles', items: [ 'Bold', 'Italic', 'Underline', 'Strike', 'Subscript', 'Superscript', '-', 'CopyFormatting', 'RemoveFormat' ] },
            { name: 'paragraph', items: [ 'NumberedList', 'BulletedList', '-', 'Outdent', 'Indent', '-', 'Blockquote', 'CreateDiv', '-', 'JustifyLeft', 'JustifyCenter', 'JustifyRight', 'JustifyBlock', '-', 'BidiLtr', 'BidiRtl', 'Language' ] },
            { name: 'links', items: [ 'Link', 'Unlink', 'Anchor' ] },
            { name: 'insert', items: [ 'Table', 'HorizontalRule', 'Smiley', 'SpecialChar', 'PageBreak'] },
            '/',
            { name: 'styles', items: [ 'Styles', 'Format', 'Font', 'FontSize' ] },
            { name: 'colors', items: [ 'TextColor', 'BGColor' ] },
            { name: 'tools', items: [ 'Maximize', 'ShowBlocks' ] },
          ],
          height: 250
        }
      },
      vw: {
        visible: false,
        title: '',
        description: '',
      }
    }
  },
  computed: {
    ...mapGetters([
      'layer1_account'
    ]),
    ...mapState(['user']),
  },
  async mounted(){
    this.wf = new Base();
    await this.wf.init();
    await this.refreshList();
  },
  methods: {
    async refreshList(){
      this.$root.loading(true);
      const list = await layer2.task.queryIdeaList(this);
      this.list = list;
      this.$root.loading(false);
    },
    
    async createNewIdeaModal(){
      this.modal.visible = true;
    },
    
    
    openHandler(){
      this.modal.title = '';
      this.modal.description = '';
      this.modal.unit = 1;
    },
    onBlur(evt) {},
    onFocus(evt) {},
    onContentDom(evt) {},
    onDialogDefinition(evt) {},

    close(){
      this.modal.visible = false;
    },

    async createNewIdea(){
      const opts = {
        title: utils.forge.util.encode64(encodeURIComponent(this.modal.title)),
        description: utils.forge.util.encode64(encodeURIComponent(this.modal.description)),
        unit: this.modal.unit,
      };
      if(!opts.title){
        return this.$root.showError("Invalid idea title");
      }
      if(!opts.description){
        return this.$root.showError("Invalid idea description");
      }
      if(!opts.unit){
        return this.$root.showError("Invalid vote price");
      }

      await layer2.task.createNewIdea(this, opts);
      this.close();
      this.$root.success("create idea success");
      await this.refreshList();
    },
    async viewIdea(row){
      this.vw.title = row.title;
      this.vw.description = row.description;
      this.vw.visible = true;
    },
    async voteIdea(row){
      try{
        await layer2.task.voteIdea(this, row, async (rs)=>{
          this.$root.success("Vote idea success");
          await this.refreshList();
        });
      }catch(e){
        this.$root.showError(e);
      }
    }
  }
};
</script>
<style lang="scss">
</style>