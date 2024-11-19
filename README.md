# Simulator Cleaner

This may be useful for developers who wish to without making many terminal requests either permanently delete or reset to their default settings multiple simulators at once.  

There is also a Ruby implementation [here](https://github.com/CallumBeaney/simulator-cleaner).


## What problem this script solves

When a new Simulator device is created, its default data is stored in a `CoreSimulator/Devices/` subdirectory. When this new device is opened for the first time, an extra 400MB - 1.5GB data is created. As different applications are tested, each device directory grows. If you remove or clear this data folder, the device becomes unopenable from Simulator's `File > Open Simulator` menu.  

The solution is to copy the chosen device type's signature data, delete the device completely, and then use that signature data to create a new device. Doing this manually is tedious.   



## Working walkthrough

__Scenario__: You want to delete your `iPhone Xʀ` and your `iPhone 13 mini` Simulators, permanently.  
To do this, you would follow the following steps, and the terminal would look like this:  

```
[user]$ simcleaner 
  [x] iPhone Xʀ                      iOS-18-0        Shutdown   1.1G
> [x] iPhone 13 mini                 iOS-18-0        Shutdown   2.8G	
  [ ] iPhone 16 Pro Max              iOS-18-0        Shutdown   17M	
  [ ] iPhone SE (3rd generation)     iOS-18-1        Shutdown   17M	
  [ ] iPhone 16 Pro Max              iOS-18-1        Shutdown   290M	
  [ ] iPad Air 13-inch (M2)          iOS-18-1        Shutdown   17M

Deleting device data:
  iPhone Xʀ [iOS-18-0]...
  iPhone 13 mini [iOS-18-0]...
Would you like to recreate these devices?
This will restore them to their default state. [y/n] no

Device data cleared. 
New devices state:

iPhone 16 Pro Max              iOS-18-0   17M	
iPhone SE (3rd generation)     iOS-18-1   17M	
iPhone 16 Pro Max              iOS-18-1   290M	
iPad Air 13-inch (M2)          iOS-18-1   17M	
[user]$ 

```

The devices would then be removed, and would no longer appear in Simulator's `File > Open Simulator > iOS 18.0` menu.    
  
__Scenario__: you now wish to clear the `iPhone 16 Pro Max` of its 290 MB of data, and restore it to its default state:

```
[user]$ simcleaner
  [ ] iPhone 16 Pro Max              iOS-18-0        Shutdown   17M	
  [ ] iPhone SE (3rd generation)     iOS-18-1        Shutdown   17M	
> [x] iPhone 16 Pro Max              iOS-18-1        Shutdown   290M	
  [ ] iPad Air 13-inch (M2)          iOS-18-1        Shutdown   17M

Deleting device data:
  iPhone 16 Pro Max [iOS-18-1]...
Would you like to recreate these devices?
This will restore them to their default state. [y/n] 

yes
Recreating following devices:
  iPhone 16 Pro Max [iOS-18-1]...
Device data cleared. 
New devices state:

iPhone 16 Pro Max              iOS-18-0   17M	
iPhone SE (3rd generation)     iOS-18-1   17M	
iPhone 16 Pro Max              iOS-18-1   17M
iPad Air 13-inch (M2)          iOS-18-1   17M	
```
  

The device has been reduced in size to the default 17MB, but still appears in and is openable from Simulator's `File > Open Simulator > iOS 18.0` menu.
