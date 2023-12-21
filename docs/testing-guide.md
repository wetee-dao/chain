
# Node Testing Guide

## Create a DAO

- Open Polkadot JS UI

- Connect to `development node --> local node`

<img src="images/Screenshot from 2023-02-18 08-30-26.png" width="500" style="padding-left: 50px;">

- Open Developer --> Extrinsics section
- From the list of available extrinsics select dao --> createDao callable 

<img src="images/Firefox_Screenshot_2023-03-09T00-48-14.726Z.png" width="500" style="padding-left: 50px;">

- Submit Transaction --> DAO successfully created

- Go to Developer --> Chain State

- Select dao --> daos for state query and press + button

<img src="images/Screenshot from 2023-02-18 08-48-31.png" width="500" style="padding-left: 50px;">

## Create a asset for DAO
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoAsset --> createAsset callable 
- input dao id 1
- input WeteeAssetsDaoAssetMeta.name mytoken
- input WeteeAssetsDaoAssetMeta.symbol  t
- input WeteeAssetsDaoAssetMeta.decimals  100
- input amount 10000
- input initDaoAsset 1000000
- Submit Transaction --> Asset successfully created

<img src="images/Firefox_Screenshot_2023-03-09T00-49-52.726Z.png" width="500" style="padding-left: 50px;">

## Create a DAO guild with gov
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoGov --> createPropose callable 
- input dao id 1
- select memberData GLOBAL
- From the list of available proposal: Call (RuntimeCall) select daoGuild --> createGuild callable 
- input dao id 1
- input name and desc and metaData
- Submit Transaction --> guild propose successfully created，after the DAO is approved by an internal vote, it can be created

<img src="images/Firefox_Screenshot_2023-03-09T02-15-32.983Z.png" width="500" style="padding-left: 50px;">

## Create a DAO project with gov
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoGov --> createPropose callable 
- input dao id 1
- select memberData GLOBAL
- From the list of available proposal: Call (RuntimeCall) select daoGuild --> createGuild callable 
- input dao id 1
- input name and desc and metaData creator
- Submit Transaction --> project propose successfully created, after the DAO is approved by an internal vote, it can be created

<img src="images/Firefox_Screenshot_2023-03-09T02-16-12.005Z.png" width="500" style="padding-left: 50px;">

## Create a DAO guild with sudo
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoSudo --> sudo callable 
- input dao id 1
- From the list of available proposal: Call (RuntimeCall) select daoGuild --> createGuild callable 
- input dao id 1
- input name and desc and metaData
- Submit Transaction --> guild successfully created

<img src="images/Firefox_Screenshot_2023-03-09T00-53-12.273Z.png" width="500" style="padding-left: 50px;">

## Create a DAO project with sudo
> creator is the init project user
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoSudo --> sudo callable 
- input dao id 1
- From the list of available proposal: Call (RuntimeCall) select daoGuild --> createGuild callable 
- input dao id 1
- input name and desc and metaData creator
- Submit Transaction --> project successfully created

<img src="images/Firefox_Screenshot_2023-03-09T00-54-03.331Z.png" width="500" style="padding-left: 50px;">

## Apply fund for project with sudo
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoSudo --> sudo callable 
- input dao id 1
- From the list of available proposal: Call (RuntimeCall) select daoGuild --> applyProjectFunds callable 
- input dao id 1
- input project id 1
- input amount
- Submit Transaction --> project will get token

<img src="images/Firefox_Screenshot_2023-03-09T00-54-54.941Z.png" width="500" style="padding-left: 50px;">

## Create project task 
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> createTask 
- input dao id 1
- input project id 1
- input name and desc and point priority
- Submit Transaction --> project task successfully created

<img src="images/Firefox_Screenshot_2023-03-09T02-02-27.851Z.png" width="500" style="padding-left: 50px;">


## join project task as assignee
> sudo must called by Alice
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> joinTask 
- input dao id 1
- input project id 1
- input task id 1
- Submit Transaction --> successfully

<img src="images/火狐截图_2023-02-18T04-24-16.236Z.png" width="500" style="padding-left: 50px;">


## new user join project with sudo (task reviewer must be other user)
> sudo must called by ALICE
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoSudo --> sudo callable 
- input dao id 1
- From the list of available proposal: Call (RuntimeCall) select daoProject --> projectJoinRequest callable 
- input dao id 1
- input project id 1
- select new user BOB
- Submit Transaction --> successfully
<img src="images/Firefox_Screenshot_2023-03-09T01-42-53.904Z.png" width="500" style="padding-left: 50px;">

## join project task as reviewer
> sudo must called by BOB
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> beTaskReview 
- input dao id 1
- input project id 1
- input task id 1
- Submit Transaction --> successfully

<img src="images/火狐截图_2023-02-18T04-16-24.622Z.png" width="500" style="padding-left: 50px;">


## Start project task 
> sudo must called by Alice
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> startTask 
- input dao id 1
- input project id 1
- input task id 1
- Submit Transaction --> project task successfully start

<img src="images/火狐截图_2023-02-18T03-47-49.440Z.png" width="500" style="padding-left: 50px;">


## Project task requset review
> sudo must called by Alice
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> requsetReview 
- input dao id 1
- input project id 1
- input task id 1
- Submit Transaction --> requset for reviewer to review

<img src="images/火狐截图_2023-02-18T04-27-57.514Z.png" width="500" style="padding-left: 50px;">

## Reviewer make task review
> sudo must called by BOB
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> requsetReview 
- input dao id 1
- input project id 1
- input task id 1
- WeteeProjectReviewOpinion option select YES
- Submit Transaction --> make task review successfully

<img src="images/火狐截图_2023-02-18T04-31-50.603Z.png" width="500" style="padding-left: 50px;">

## Project task done
> sudo must called by Alice
- Open Developer --> Extrinsics section
- From the list of available extrinsics select daoProject --> requsetReview 
- input dao id 1
- input project id 1
- input task id 1
- Submit Transaction --> rtask done

<img src="images/火狐截图_2023-02-18T04-34-16.726Z.png" width="500" style="padding-left: 50px;">
