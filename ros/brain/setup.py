from setuptools import setup

package_name = 'brain'

setup(
    name=package_name,
    version='0.0.0',
    packages=[package_name],
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='aaf',
    maintainer_email='alonsofonseca.angel@gmail.com',
    description='TODO: Package description',
    license='TODO: License declaration',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'main = brain.brain_main:main',
            'talker = brain.publisher_member_function:main',
            'listener = brain.subscriber_member_function:main',
        ],
    },
)
