<script setup lang="ts">
import {
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub
} from '@/components/ui/sidebar'
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger
} from '@/components/ui/collapsible'
import {
  Minus,
  Plus,
  Database,
  Copy,
  MoreHorizontal,
  DatabaseBackup,
  DatabaseZap,
  AlertCircle,
  RotateCcw
} from 'lucide-vue-next'
import {Alert, AlertDescription, AlertTitle} from '@/components/ui/alert'



import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import {computed, watchEffect} from 'vue'
import {Skeleton} from '@/components/ui/skeleton'
import {Button} from '@/components/ui/button'
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from '@/components/ui/tooltip'
import {useDbStore} from "@/stores/dbStore.ts";
import SchemaList from "@/components/dataSources/SchemaList.vue";
import {useAppStore} from "@/stores/appStore.ts";

const props = defineProps<{ dsName: string; search: string }>()
const dbStore = useDbStore();
watchEffect(()=>{
  if(props.dsName && props.dsName !=''){
      dbStore.loadDbs(props.dsName)
  }
})

const currentDbs = computed(() => {
  if (props.search !== '') {
    return dbStore.dbs.filter((db) => db.includes(props.search))
  }
  return dbStore.dbs
})
const appStore = useAppStore()
</script>

<template>
  <template v-if="!dbStore.isLoadingDbs && !dbStore.hasConexionErrors">
    <SidebarMenuItem v-for="db in currentDbs" :key="db">
      <Collapsible class="group/collapsible">
        <CollapsibleTrigger as-child>
          <SidebarMenuButton>
            <Database class="text-blue-600"/>
            <!--           <span class="font-semibold"> {{ db }}</span>-->
            {{ db }}
            <Plus class="ml-auto group-data-[state=open]/collapsible:hidden"/>
            <Minus class="ml-auto group-data-[state=closed]/collapsible:hidden"/>
          </SidebarMenuButton>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <SidebarMenuSub>
            <SchemaList :ds-name="props.dsName" :db-name="db"/>
          </SidebarMenuSub>
        </CollapsibleContent>
      </Collapsible>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <SidebarMenuAction show-on-hover>
            <MoreHorizontal/>
            <span class="sr-only">More</span>
          </SidebarMenuAction>
        </DropdownMenuTrigger>
        <DropdownMenuContent class="w-26 rounded-lg" side="bottom" align="end">
          <DropdownMenuItem @click="appStore.openBackup(props.dsName, db)">
            <DatabaseBackup class="text-muted-foreground"/>
            <span>Backup</span>
          </DropdownMenuItem>
          <DropdownMenuSeparator/>
          <DropdownMenuItem >
            <DatabaseZap class="text-muted-foreground"/>
            <span>Restore</span>
          </DropdownMenuItem>
          <DropdownMenuSeparator/>
          <DropdownMenuItem >
            <Copy class="text-muted-foreground"/>
            <span>Clone to</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  </template>
  <template v-else-if="dbStore.isLoadingDbs && !dbStore.hasConexionErrors">
    <div class="flex items-center space-x-4 mb-3">
      <Skeleton class="h-6 w-6 rounded-full"/>
      <div class="space-y-2">
        <Skeleton class="h-4 w-[200px]"/>
      </div>
      <Skeleton class="h-3 w-3 rounded-full"/>
    </div>
    <div class="flex items-center space-x-4 mb-3">
      <Skeleton class="h-6 w-6 rounded-full"/>
      <div class="space-y-2">
        <Skeleton class="h-4 w-[200px]"/>
      </div>
      <Skeleton class="h-3 w-3 rounded-full"/>
    </div>
    <div class="flex items-center space-x-4 mb-3">
      <Skeleton class="h-6 w-6 rounded-full"/>
      <div class="space-y-2">
        <Skeleton class="h-4 w-[200px]"/>
      </div>
      <Skeleton class="h-3 w-3 rounded-full"/>
    </div>
  </template>
  <template v-else>
    <Alert variant="destructive">
      <AlertCircle class="w-4 h-4"/>
      <AlertTitle>Error</AlertTitle>
      <AlertDescription class="flex  ">
        <span>Conexion refused, check datasource params.</span>

        <TooltipProvider disable-closing-trigger>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button variant="outline" size="icon" class="text-muted-foreground/90" @click="dbStore.loadDbs(props.dsName)">
                <RotateCcw/>
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Retry</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </AlertDescription>
    </Alert>
  </template>
</template>

<style scoped></style>
